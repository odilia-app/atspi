use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use regex::Regex;
use std::{
	collections::HashMap,
	str::FromStr,
};
use syn::{
    self, fold::Fold, parse_quote, spanned::Spanned, AttributeArgs, Error, FnArg, Ident, ItemTrait,
    NestedMeta, ReturnType, TraitItemMethod, Type,
};

use crate::utils::*;

struct AsyncOpts {
    blocking: bool,
    usage: TokenStream,
    wait: TokenStream,
}

impl AsyncOpts {
    fn new(blocking: bool) -> Self {
        let (usage, wait) = if blocking {
            (quote! {}, quote! {})
        } else {
            (quote! { async }, quote! { .await })
        };
        Self {
            blocking,
            usage,
            wait,
        }
    }
}

pub fn expand(input: ItemTrait) -> Result<TokenStream, Error> {
		let async_trait_name = format!("{}", input.ident);
		let trait_name = format!("{}Blocking", input.ident);
    let blocking_trait = create_trait(&input, &trait_name, true)?;
    let async_trait = create_trait(&input, &async_trait_name, false)?;
		let blocking_impl = create_proxy_trait_impl(&input, &async_trait_name, true)?;
		let async_impl = create_proxy_trait_impl(&input, &async_trait_name, false)?;

    Ok(quote! {
        #blocking_trait

				#blocking_impl

				#[async_trait]
        #async_trait

				#[async_trait]
				#async_impl
    })
}

pub fn create_proxy_trait_impl(
    input: &ItemTrait,
    trait_name: &str,
    blocking: bool,
) -> Result<TokenStream, Error> {
    let zbus = zbus_path();
		let proxy_name_string = if blocking {
			format!("{trait_name}ProxyBlocking")
		} else {
			format!("{trait_name}Proxy")
		};
		let trait_impl_name_string = if blocking {
			format!("{trait_name}Blocking")
		} else {
			format!("{trait_name}")
		};
		let trait_impl_name = TokenStream::from_str(&trait_impl_name_string).expect("Could not create token stream from \"{trait_impl_name_string}\"");
		let proxy_name = TokenStream::from_str(&proxy_name_string)?;
    let other_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let trait_name = Ident::new(trait_name, Span::call_site());
    let ident = input.ident.to_string();
    let mut methods = TokenStream::new();
    let mut trait_methods = TokenStream::new();
    let mut stream_types = TokenStream::new();
    let mut has_properties = false;
    let mut uncached_properties: Vec<String> = vec![];

    let async_opts = AsyncOpts::new(blocking);

    for i in input.items.iter() {
        if let syn::TraitItem::Method(m) = i {
            let method_name = m.sig.ident.to_string();
            let attrs = parse_item_attributes(&m.attrs, "dbus_proxy")?;
            let property_attrs = attrs.iter().find_map(|x| match x {
                ItemAttribute::Property(v) => Some(v),
                _ => None,
            });
            let is_property = property_attrs.is_some();
            let is_signal = attrs.iter().any(|x| x.is_signal());
            let has_inputs = m.sig.inputs.len() > 1;
            let member_name = attrs
                .iter()
                .find_map(|x| match x {
                    ItemAttribute::Name(n) => Some(n.to_string()),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    pascal_case(if is_property && has_inputs {
                        assert!(method_name.starts_with("set_"));
                        &method_name[4..]
                    } else {
                        &method_name
                    })
                });
            let m = if let Some(prop_attrs) = property_attrs {
                assert!(is_property);
                has_properties = true;
                let emits_changed_signal = PropertyEmitsChangedSignal::parse_from_attrs(prop_attrs);
                if let PropertyEmitsChangedSignal::False = emits_changed_signal {
                    uncached_properties.push(member_name.clone());
                }
                gen_proxy_trait_impl_property(
                    &member_name,
                    &method_name,
										&proxy_name_string,
                    m,
                    &async_opts,
                    emits_changed_signal,
                )
            } else if is_signal {
                let method = gen_proxy_signal(
                    &trait_name,
                    &member_name,
                    &method_name,
                    m,
                    &async_opts,
                );
                method
            } else {
                gen_proxy_trait_method_impl(&member_name, &method_name, &proxy_name_string, m, &async_opts)
            };
            methods.extend(m);
        }
    }

    let AsyncOpts { usage, wait, .. } = async_opts;
    let (proxy_struct, connection, builder) = if blocking {
        let connection = quote! { #zbus::blocking::Connection };
        let proxy = quote! { #zbus::blocking::Proxy };
        let builder = quote! { #zbus::blocking::ProxyBuilder };

        (proxy, connection, builder)
    } else {
        let connection = quote! { #zbus::Connection };
        let proxy = quote! { #zbus::Proxy };
        let builder = quote! { #zbus::ProxyBuilder };

        (proxy, connection, builder)
    };

		Ok(quote! {
				impl<'c> #trait_impl_name for #proxy_name<'c> {
					type Error = zbus::Error;
					#methods
				}
		})
}
pub fn create_trait(
    input: &ItemTrait,
    trait_name: &str,
    blocking: bool,
) -> Result<TokenStream, Error> {
    let zbus = zbus_path();

    let other_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let trait_name = Ident::new(trait_name, Span::call_site());
    let ident = input.ident.to_string();
    let mut methods = TokenStream::new();
    let mut trait_methods = TokenStream::new();
    let mut stream_types = TokenStream::new();
    let mut has_properties = false;
    let mut uncached_properties: Vec<String> = vec![];

    let async_opts = AsyncOpts::new(blocking);

    for i in input.items.iter() {
        if let syn::TraitItem::Method(m) = i {
            let method_name = m.sig.ident.to_string();
            let attrs = parse_item_attributes(&m.attrs, "dbus_proxy")?;
            let property_attrs = attrs.iter().find_map(|x| match x {
                ItemAttribute::Property(v) => Some(v),
                _ => None,
            });
            let is_property = property_attrs.is_some();
            let is_signal = attrs.iter().any(|x| x.is_signal());
            let has_inputs = m.sig.inputs.len() > 1;
            let member_name = attrs
                .iter()
                .find_map(|x| match x {
                    ItemAttribute::Name(n) => Some(n.to_string()),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    pascal_case(if is_property && has_inputs {
                        assert!(method_name.starts_with("set_"));
                        &method_name[4..]
                    } else {
                        &method_name
                    })
                });
            let m = if let Some(prop_attrs) = property_attrs {
                assert!(is_property);
                has_properties = true;
                let emits_changed_signal = PropertyEmitsChangedSignal::parse_from_attrs(prop_attrs);
                if let PropertyEmitsChangedSignal::False = emits_changed_signal {
                    uncached_properties.push(member_name.clone());
                }
                gen_trait_property(
                    &member_name,
                    &method_name,
                    m,
                    &async_opts,
                    emits_changed_signal,
                )
            } else {
                gen_trait_method_signature(&member_name, &method_name, m, &async_opts)
            };
            trait_methods.extend(m);
        }
    }

    let AsyncOpts { usage, wait, .. } = async_opts;
    let (proxy_struct, connection, builder) = if blocking {
        let connection = quote! { #zbus::blocking::Connection };
        let proxy = quote! { #zbus::blocking::Proxy };
        let builder = quote! { #zbus::blocking::ProxyBuilder };

        (proxy, connection, builder)
    } else {
        let connection = quote! { #zbus::Connection };
        let proxy = quote! { #zbus::Proxy };
        let builder = quote! { #zbus::ProxyBuilder };

        (proxy, connection, builder)
    };

		if blocking {
			Ok(quote! {
					pub trait #trait_name {
						type Error;
						#trait_methods
					}
			})
		} else {
			Ok(quote! {
					pub trait #trait_name {
						type Error;
						#trait_methods
					}
			})
		}
}

// TODO: this is sketchy as all hell
// it replaces all mentions of zbus::Result with the Generic std::result::Result, then, adds the Self::Error error type to the second part of the generic
// finally, it replaces all mentions of (String, zbus :: zvairnat :: OwnedObjectPath) with &Self.
// this menas that implementors will need to return a borrowed value of the same type to comply with the type system.
// unsure if this will hold up over time.
fn genericize_method_return_type(rt: &ReturnType) -> TokenStream {
	let original = format!("{}", rt.to_token_stream());
	let mut generic_result = original.replace("zbus :: Result", "std :: result :: Result");
	let end_of_str = generic_result.len();
	generic_result.insert_str(end_of_str-2, ", Self :: Error");
	let mut generic_impl = generic_result.replace("(String, zbus :: zvariant :: OwnedObjectPath)", "Self");
	generic_impl.push_str(" where Self: Sized");
	TokenStream::from_str(&generic_impl).expect("Could not genericize zbus method/property/signal. Attempted to turn \"{generic_result}\" into a TokenStream.")
}

fn gen_trait_method_signature(
    method_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let args: Vec<_> = m
        .sig
        .inputs
        .iter()
        .filter_map(typed_arg)
        .filter_map(pat_ident)
        .collect();
    let attrs = parse_item_attributes(&m.attrs, "dbus_proxy").unwrap();
    let async_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::AsyncObject(o) => Some(o.clone()),
        _ => None,
    });
    let blocking_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::BlockingObject(o) => Some(o.clone()),
        _ => None,
    });
    let proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::Object(o) => {
            if *blocking {
                // FIXME: for some reason Rust doesn't let us move `blocking_proxy_object` so we've to clone.
                blocking_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{o}ProxyBlocking")))
            } else {
                async_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{o}Proxy")))
            }
        }
        _ => None,
    });
    let no_reply = attrs.iter().any(|x| matches!(x, ItemAttribute::NoReply));
    let no_autostart = attrs
        .iter()
        .any(|x| matches!(x, ItemAttribute::NoAutoStart));
    let allow_interactive_auth = attrs
        .iter()
        .any(|x| matches!(x, ItemAttribute::AllowInteractiveAuth));

    let method_flags = match (no_reply, no_autostart, allow_interactive_auth) {
        (true, false, false) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::NoReplyExpected
        ))),
        (false, true, false) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::NoAutoStart
        ))),
        (false, false, true) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::AllowInteractiveAuth
        ))),

        (true, true, false) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected | zbus::MethodFlags::NoAutoStart
        )),
        (true, false, true) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected | zbus::MethodFlags::AllowInteractiveAuth
        )),
        (false, true, true) => Some(quote!(
            zbus::MethodFlags::NoAutoStart | zbus::MethodFlags::AllowInteractiveAuth
        )),

        (true, true, true) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected
                | zbus::MethodFlags::NoAutoStart
                | zbus::MethodFlags::AllowInteractiveAuth
        )),
        _ => None,
    };

    let method = Ident::new(snake_case_name, Span::call_site());
    let inputs = &m.sig.inputs;
    let mut generics = m.sig.generics.clone();
    let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
    for param in generics
        .params
        .iter()
        .filter(|a| matches!(a, syn::GenericParam::Type(_)))
    {
        let is_input_type = inputs.iter().any(|arg| {
            // FIXME: We want to only require `Serialize` from input types and `DeserializeOwned`
            // from output types but since we don't have type introspection, we employ this
            // workaround of regex matching on string reprepresention of the the types to figure out
            // which generic types are input types.
            if let FnArg::Typed(pat) = arg {
                let pattern = format!("& *{}", param.to_token_stream());
                let regex = Regex::new(&pattern).unwrap();
                regex.is_match(&pat.ty.to_token_stream().to_string())
            } else {
                false
            }
        });
        let serde_bound: TokenStream = if is_input_type {
            parse_quote!(#zbus::export::serde::ser::Serialize)
        } else {
            parse_quote!(#zbus::export::serde::de::DeserializeOwned)
        };
        where_clause.predicates.push(parse_quote!(
            #param: #serde_bound + #zbus::zvariant::Type
        ));
    }
    let (_, ty_generics, where_clause) = generics.split_for_impl();

		let body = if args.len() == 1 {
				// Wrap single arg in a tuple so if it's a struct/tuple itself, zbus will only remove
				// the '()' from the signature that we add and not the actual intended ones.
				let arg = &args[0];
				quote! {
						&(#arg,)
				}
		} else {
				quote! {
						&(#(#args),*)
				}
		};

		let output = genericize_method_return_type(&m.sig.output);
		let signature = quote! {
				fn #method(#inputs) #output
		};

		quote! {
				#(#other_attrs)*
				#usage #signature;
		}
}
fn gen_proxy_trait_method_impl(
    method_name: &str,
    snake_case_name: &str,
		proxy_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let args: Vec<_> = m
        .sig
        .inputs
        .iter()
        .filter_map(typed_arg)
        .filter_map(pat_ident)
        .collect();
    let attrs = parse_item_attributes(&m.attrs, "dbus_proxy").unwrap();
    let async_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::AsyncObject(o) => Some(o.clone()),
        _ => None,
    });
    let blocking_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::BlockingObject(o) => Some(o.clone()),
        _ => None,
    });
    let proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::Object(o) => {
            if *blocking {
                // FIXME: for some reason Rust doesn't let us move `blocking_proxy_object` so we've to clone.
                blocking_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{o}ProxyBlocking")))
            } else {
                async_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{o}Proxy")))
            }
        }
        _ => None,
    });
    let no_reply = attrs.iter().any(|x| matches!(x, ItemAttribute::NoReply));
    let no_autostart = attrs
        .iter()
        .any(|x| matches!(x, ItemAttribute::NoAutoStart));
    let allow_interactive_auth = attrs
        .iter()
        .any(|x| matches!(x, ItemAttribute::AllowInteractiveAuth));

    let method_flags = match (no_reply, no_autostart, allow_interactive_auth) {
        (true, false, false) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::NoReplyExpected
        ))),
        (false, true, false) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::NoAutoStart
        ))),
        (false, false, true) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::AllowInteractiveAuth
        ))),

        (true, true, false) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected | zbus::MethodFlags::NoAutoStart
        )),
        (true, false, true) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected | zbus::MethodFlags::AllowInteractiveAuth
        )),
        (false, true, true) => Some(quote!(
            zbus::MethodFlags::NoAutoStart | zbus::MethodFlags::AllowInteractiveAuth
        )),

        (true, true, true) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected
                | zbus::MethodFlags::NoAutoStart
                | zbus::MethodFlags::AllowInteractiveAuth
        )),
        _ => None,
    };

    let method = Ident::new(snake_case_name, Span::call_site());
    let inputs = &m.sig.inputs;
    let mut generics = m.sig.generics.clone();
    let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
    for param in generics
        .params
        .iter()
        .filter(|a| matches!(a, syn::GenericParam::Type(_)))
    {
        let is_input_type = inputs.iter().any(|arg| {
            // FIXME: We want to only require `Serialize` from input types and `DeserializeOwned`
            // from output types but since we don't have type introspection, we employ this
            // workaround of regex matching on string reprepresention of the the types to figure out
            // which generic types are input types.
            if let FnArg::Typed(pat) = arg {
                let pattern = format!("& *{}", param.to_token_stream());
                let regex = Regex::new(&pattern).unwrap();
                regex.is_match(&pat.ty.to_token_stream().to_string())
            } else {
                false
            }
        });
        let serde_bound: TokenStream = if is_input_type {
            parse_quote!(#zbus::export::serde::ser::Serialize)
        } else {
            parse_quote!(#zbus::export::serde::de::DeserializeOwned)
        };
        where_clause.predicates.push(parse_quote!(
            #param: #serde_bound + #zbus::zvariant::Type
        ));
    }
    let (_, ty_generics, where_clause) = generics.split_for_impl();

		let body = if args.len() == 1 {
				// Wrap single arg in a tuple so if it's a struct/tuple itself, zbus will only remove
				// the '()' from the signature that we add and not the actual intended ones.
				let arg = &args[0];
				quote! {
						#arg
				}
		} else if args.len() == 0 {
			quote! {}
		} else {
				quote! {
						#(#args),*
				}
		};

		let output = genericize_method_return_type(&m.sig.output);
		let signature = quote! {
				fn #method(#inputs) #output
		};

		let output_str = format!("{output}");
		let proxy = TokenStream::from_str(proxy_name).expect("Could not create token stream from \"{proxy_name}\"");
		if output_str.contains("Result < Self") {
			quote! {
				#(#other_attrs)*
				#usage #signature {
					let object_pair = self.#method(#body)#wait?;
					let conn = self.connection().clone();
					#proxy::builder(&conn)
						.path(object_pair.1)?
						.destination(object_pair.0)?
						.build()
						#wait
				}
			}
		} else if output_str.contains("< Vec < Self") {
			quote! {
				#(#other_attrs)*
				#usage #signature {
					let vec_of_object_pairs = self.#method()#wait?;
					let mut vec_self = Vec::new();
					let conn = self.connection().clone();
					for object_pair in vec_of_object_pairs {
						let proxy = #proxy::builder(&conn)
							.path(object_pair.1)?
							.destination(object_pair.0)?
							.build()
							#wait?;
						vec_self.push(proxy);
					}
					Ok(vec_self)
				}
			}
	} else {
			if inputs.len() >= 1 {
				quote! {
						#(#other_attrs)*
						#usage #signature {
							self.#method(#body)#wait
						}
				}
			} else {
				quote! {
						#(#other_attrs)*
						#usage #signature {
							self.#method()#wait
						}
				}
			}
	}
}
fn gen_proxy_method_call(
    method_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let args: Vec<_> = m
        .sig
        .inputs
        .iter()
        .filter_map(typed_arg)
        .filter_map(pat_ident)
        .collect();
    let attrs = parse_item_attributes(&m.attrs, "dbus_proxy").unwrap();
    let async_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::AsyncObject(o) => Some(o.clone()),
        _ => None,
    });
    let blocking_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::BlockingObject(o) => Some(o.clone()),
        _ => None,
    });
    let proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::Object(o) => {
            if *blocking {
                // FIXME: for some reason Rust doesn't let us move `blocking_proxy_object` so we've to clone.
                blocking_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{o}ProxyBlocking")))
            } else {
                async_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{o}Proxy")))
            }
        }
        _ => None,
    });
    let no_reply = attrs.iter().any(|x| matches!(x, ItemAttribute::NoReply));
    let no_autostart = attrs
        .iter()
        .any(|x| matches!(x, ItemAttribute::NoAutoStart));
    let allow_interactive_auth = attrs
        .iter()
        .any(|x| matches!(x, ItemAttribute::AllowInteractiveAuth));

    let method_flags = match (no_reply, no_autostart, allow_interactive_auth) {
        (true, false, false) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::NoReplyExpected
        ))),
        (false, true, false) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::NoAutoStart
        ))),
        (false, false, true) => Some(quote!(::std::convert::Into::into(
            zbus::MethodFlags::AllowInteractiveAuth
        ))),

        (true, true, false) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected | zbus::MethodFlags::NoAutoStart
        )),
        (true, false, true) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected | zbus::MethodFlags::AllowInteractiveAuth
        )),
        (false, true, true) => Some(quote!(
            zbus::MethodFlags::NoAutoStart | zbus::MethodFlags::AllowInteractiveAuth
        )),

        (true, true, true) => Some(quote!(
            zbus::MethodFlags::NoReplyExpected
                | zbus::MethodFlags::NoAutoStart
                | zbus::MethodFlags::AllowInteractiveAuth
        )),
        _ => None,
    };

    let method = Ident::new(snake_case_name, Span::call_site());
    let inputs = &m.sig.inputs;
    let mut generics = m.sig.generics.clone();
    let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
    for param in generics
        .params
        .iter()
        .filter(|a| matches!(a, syn::GenericParam::Type(_)))
    {
        let is_input_type = inputs.iter().any(|arg| {
            // FIXME: We want to only require `Serialize` from input types and `DeserializeOwned`
            // from output types but since we don't have type introspection, we employ this
            // workaround of regex matching on string reprepresention of the the types to figure out
            // which generic types are input types.
            if let FnArg::Typed(pat) = arg {
                let pattern = format!("& *{}", param.to_token_stream());
                let regex = Regex::new(&pattern).unwrap();
                regex.is_match(&pat.ty.to_token_stream().to_string())
            } else {
                false
            }
        });
        let serde_bound: TokenStream = if is_input_type {
            parse_quote!(#zbus::export::serde::ser::Serialize)
        } else {
            parse_quote!(#zbus::export::serde::de::DeserializeOwned)
        };
        where_clause.predicates.push(parse_quote!(
            #param: #serde_bound + #zbus::zvariant::Type
        ));
    }
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    if let Some(proxy_name) = proxy_object {
        let proxy = Ident::new(&proxy_name, Span::call_site());
        let signature = quote! {
            fn #method#ty_generics(#inputs) -> #zbus::Result<#proxy<'c>>
            #where_clause
        };

        quote! {
            #(#other_attrs)*
            pub #usage #signature {
                let object_path: #zbus::zvariant::OwnedObjectPath =
                    self.0.call(
                        #method_name,
                        &(#(#args),*),
                    )
                    #wait?;
                #proxy::builder(&self.0.connection())
                    .path(object_path)?
                    .build()
                    #wait
            }
        }
    } else {
        let body = if args.len() == 1 {
            // Wrap single arg in a tuple so if it's a struct/tuple itself, zbus will only remove
            // the '()' from the signature that we add and not the actual intended ones.
            let arg = &args[0];
            quote! {
                &(#arg,)
            }
        } else {
            quote! {
                &(#(#args),*)
            }
        };

        let output = &m.sig.output;
        let signature = quote! {
            fn #method#ty_generics(#inputs) #output
            #where_clause
        };

        if let Some(method_flags) = method_flags {
            if no_reply {
                quote! {
                    #(#other_attrs)*
                    pub #usage #signature {
                        self.0.call_with_flags::<_, _, ()>(#method_name, #method_flags, #body)#wait?;
                        ::std::result::Result::Ok(())
                    }
                }
            } else {
                quote! {
                    #(#other_attrs)*
                    pub #usage #signature {
                        let reply = self.0.call_with_flags(#method_name, #method_flags, #body)#wait?;

                        // SAFETY: This unwrap() cannot fail due to the guarantees in
                        // call_with_flags, which can only return Ok(None) if the
                        // NoReplyExpected is set. By not passing NoReplyExpected,
                        // we are guaranteed to get either an Err variant (handled
                        // in the previous statement) or Ok(Some(T)) which is safe to
                        // unwrap
                        ::std::result::Result::Ok(reply.unwrap())
                    }
                }
            }
        } else {
            quote! {
                #(#other_attrs)*
                pub #usage #signature {
                    let reply = self.0.call(#method_name, #body)#wait?;
                    ::std::result::Result::Ok(reply)
                }
            }
        }
    }
}

/// Standard annotation `org.freedesktop.DBus.Property.EmitsChangedSignal`.
///
/// See <https://dbus.freedesktop.org/doc/dbus-specification.html#introspection-format>.
#[derive(Debug)]
enum PropertyEmitsChangedSignal {
    True,
    Invalidates,
    Const,
    False,
}

impl Default for PropertyEmitsChangedSignal {
    fn default() -> Self {
        PropertyEmitsChangedSignal::True
    }
}

impl PropertyEmitsChangedSignal {
    /// Macro property attribute key, like `#[dbus_proxy(property(emits_changed_signal = "..."))]`.
    const ATTRIBUTE_KEY: &'static str = "emits_changed_signal";

    /// Parse the value from macro attributes.
    fn parse_from_attrs(attrs: &HashMap<String, String>) -> Self {
        attrs
            .get(Self::ATTRIBUTE_KEY)
            .map(|val| match val.as_str() {
                "true" => PropertyEmitsChangedSignal::True,
                "invalidates" => PropertyEmitsChangedSignal::Invalidates,
                "const" => PropertyEmitsChangedSignal::Const,
                "false" => PropertyEmitsChangedSignal::False,
                x => panic!("Invalid attribute '{} = {}'", Self::ATTRIBUTE_KEY, x),
            })
            .unwrap_or_default()
    }
}

fn gen_trait_property(
    property_name: &str,
    method_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
    emits_changed_signal: PropertyEmitsChangedSignal,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let method = Ident::new(method_name, Span::call_site());
		let signature = &m.sig;
    let inputs = &m.sig.inputs;
    let output = genericize_method_return_type(&m.sig.output);
    if signature.inputs.len() > 1 {
        quote! {
            #(#other_attrs)*
            #usage #method(#inputs) #output;
        }
    } else {
        quote! {
            #(#other_attrs)*
            #usage fn #method(#inputs) #output;
        }
    }
}
fn gen_proxy_trait_impl_property(
    property_name: &str,
    method_name: &str,
		proxy_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
    emits_changed_signal: PropertyEmitsChangedSignal,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
		let inputs = &m.sig.inputs;
    let output = genericize_method_return_type(&m.sig.output);
    let signature = &m.sig;
		let method = TokenStream::from_str(method_name).expect("Could not convert \"{method_name}\" into a token stream");
    if signature.inputs.len() > 1 {
        let value = pat_ident(typed_arg(signature.inputs.last().unwrap()).unwrap()).unwrap();
        quote! {
            #(#other_attrs)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                ::std::result::Result::Ok(self.0.set_property(#property_name, #value)#wait?)
            }
        }
    } else {
        // This should fail to compile only if the return type is wrong,
        // so use that as the span.
        let body_span = if let ReturnType::Type(_, ty) = &signature.output {
            ty.span()
        } else {
            signature.span()
        };
				let output_str = format!("{}", output);
				let proxy = TokenStream::from_str(proxy_name).expect("Could not create token stream from \"{proxy_name}\"");
				let body = if output_str.contains("Result < Self,") {
					quote! {
						let object_pair = self.#method()#wait?;
						let conn = self.connection().clone();
						#proxy::builder(&conn)
							.path(object_pair.1)?
							.destination(object_pair.0)?
							.build()
							#wait
					}
				} else {
					quote! {
            self.#method()#wait
					}
        };
        let ret_type = if let ReturnType::Type(_, ty) = &signature.output {
            Some(ty)
        } else {
            None
        };

        let (proxy_name, prop_stream) = if *blocking {
            (
                "zbus::blocking::Proxy",
                quote! { #zbus::blocking::PropertyIterator },
            )
        } else {
            ("zbus::Proxy", quote! { #zbus::PropertyStream })
        };

				if inputs.len() >= 1 {
					quote! {
							#(#other_attrs)*
							#usage fn #method(#inputs) #output {
									#body
							}
					}
				} else {
					quote! {
							#(#other_attrs)*
							#usage fn #method(&self) #output {
									#body
							}
					}
				}
    }
}
fn gen_proxy_property(
    property_name: &str,
    method_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
    emits_changed_signal: PropertyEmitsChangedSignal,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let signature = &m.sig;
    if signature.inputs.len() > 1 {
        let value = pat_ident(typed_arg(signature.inputs.last().unwrap()).unwrap()).unwrap();
        quote! {
            #(#other_attrs)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                ::std::result::Result::Ok(self.0.set_property(#property_name, #value)#wait?)
            }
        }
    } else {
        // This should fail to compile only if the return type is wrong,
        // so use that as the span.
        let body_span = if let ReturnType::Type(_, ty) = &signature.output {
            ty.span()
        } else {
            signature.span()
        };
        let body = quote_spanned! {body_span =>
            ::std::result::Result::Ok(self.0.get_property(#property_name)#wait?)
        };
        let ret_type = if let ReturnType::Type(_, ty) = &signature.output {
            Some(ty)
        } else {
            None
        };

        let (proxy_name, prop_stream) = if *blocking {
            (
                "zbus::blocking::Proxy",
                quote! { #zbus::blocking::PropertyIterator },
            )
        } else {
            ("zbus::Proxy", quote! { #zbus::PropertyStream })
        };

        let receive_method = match emits_changed_signal {
            PropertyEmitsChangedSignal::True | PropertyEmitsChangedSignal::Invalidates => {
                let (_, ty_generics, where_clause) = m.sig.generics.split_for_impl();
                let receive = format_ident!("receive_{}_changed", method_name);
                let gen_doc = format!(
                    "Create a stream for the `{property_name}` property changes. \
                This is a convenient wrapper around [`{proxy_name}::receive_property_changed`]."
                );
                quote! {
                    #[doc = #gen_doc]
                    pub #usage fn #receive#ty_generics(
                        &self
                    ) -> #prop_stream<'c, <#ret_type as #zbus::ResultAdapter>::Ok>
                    #where_clause
                    {
                        self.0.receive_property_changed(#property_name)#wait
                    }
                }
            }
            PropertyEmitsChangedSignal::False | PropertyEmitsChangedSignal::Const => {
                quote! {}
            }
        };

        let cached_getter_method = match emits_changed_signal {
            PropertyEmitsChangedSignal::True
            | PropertyEmitsChangedSignal::Invalidates
            | PropertyEmitsChangedSignal::Const => {
                let cached_getter = format_ident!("cached_{}", method_name);
                let cached_doc = format!(
                    " Get the cached value of the `{property_name}` property, or `None` if the property is not cached.",
                );
                quote! {
                    #[doc = #cached_doc]
                    pub fn #cached_getter(&self) -> ::std::result::Result<
                        ::std::option::Option<<#ret_type as #zbus::ResultAdapter>::Ok>,
                        <#ret_type as #zbus::ResultAdapter>::Err>
                    {
                        self.0.cached_property(#property_name).map_err(::std::convert::Into::into)
                    }
                }
            }
            PropertyEmitsChangedSignal::False => quote! {},
        };

        quote! {
            #(#other_attrs)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                #body
            }

            #cached_getter_method

            #receive_method
        }
    }
}

struct SetLifetimeS;

impl Fold for SetLifetimeS {
    fn fold_type_reference(&mut self, node: syn::TypeReference) -> syn::TypeReference {
        let mut t = syn::fold::fold_type_reference(self, node);
        t.lifetime = Some(syn::Lifetime::new("'s", Span::call_site()));
        t
    }

    fn fold_lifetime(&mut self, _node: syn::Lifetime) -> syn::Lifetime {
        syn::Lifetime::new("'s", Span::call_site())
    }
}

fn gen_proxy_signal(
    proxy_name: &Ident,
    signal_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let input_types: Vec<Box<Type>> = m
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(p) => Some(p.ty.clone()),
            _ => None,
        })
        .collect();
    let input_types_s: Vec<_> = SetLifetimeS
        .fold_signature(m.sig.clone())
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(p) => Some(p.ty.clone()),
            _ => None,
        })
        .collect();
    let args: Vec<Ident> = m
        .sig
        .inputs
        .iter()
        .filter_map(typed_arg)
        .filter_map(|arg| pat_ident(arg).cloned())
        .collect();
    let args_nth: Vec<Literal> = args
        .iter()
        .enumerate()
        .map(|(i, _)| Literal::usize_unsuffixed(i))
        .collect();

    let mut generics = m.sig.generics.clone();
    let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
    for param in generics
        .params
        .iter()
        .filter(|a| matches!(a, syn::GenericParam::Type(_)))
    {
        where_clause
                .predicates
                .push(parse_quote!(#param: #zbus::export::serde::de::Deserialize<'s> + #zbus::zvariant::Type + ::std::fmt::Debug));
    }
    generics.params.push(parse_quote!('s));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (
        proxy_path,
        receive_signal_link,
        receive_signal_with_args_link,
        trait_name,
        trait_link,
        signal_type,
    ) = if *blocking {
        (
            "zbus::blocking::Proxy",
            "https://docs.rs/zbus/latest/zbus/blocking/struct.Proxy.html#method.receive_signal",
            "https://docs.rs/zbus/latest/zbus/blocking/struct.Proxy.html#method.receive_signal_with_args",
            "Iterator",
            "https://doc.rust-lang.org/std/iter/trait.Iterator.html",
            quote! { blocking::SignalIterator },
        )
    } else {
        (
            "zbus::Proxy",
            "https://docs.rs/zbus/latest/zbus/struct.Proxy.html#method.receive_signal",
            "https://docs.rs/zbus/latest/zbus/struct.Proxy.html#method.receive_signal_with_args",
            "Stream",
            "https://docs.rs/futures/0.3.15/futures/stream/trait.Stream.html",
            quote! { SignalStream },
        )
    };
    let receiver_name = format_ident!("receive_{snake_case_name}");
    let receiver_with_args_name = format_ident!("receive_{snake_case_name}_with_args");
    let stream_name = format_ident!("{signal_name}{trait_name}");
    let signal_args = format_ident!("{signal_name}Args");
    let signal_name_ident = format_ident!("{signal_name}");

    let receive_gen_doc = format!(
        "Create a stream that receives `{signal_name}` signals.\n\
            \n\
            This a convenient wrapper around [`{proxy_path}::receive_signal`]({receive_signal_link}).",
    );
    let receive_with_args_gen_doc = format!(
        "Create a stream that receives `{signal_name}` signals.\n\
            \n\
            This a convenient wrapper around [`{proxy_path}::receive_signal_with_args`]({receive_signal_with_args_link}).",
    );
    let receive_signal_with_args = if args.is_empty() {
        quote!()
    } else {
        quote! {
            #[doc = #receive_with_args_gen_doc]
            #(#other_attrs)*
            pub #usage fn #receiver_with_args_name(&self, args: &[(u8, &str)]) -> #zbus::Result<#stream_name<'c>>
            {
                self.receive_signal_with_args(#signal_name, args)#wait.map(#stream_name)
            }
        }
    };
    let receive_signal = quote! {
        #[doc = #receive_gen_doc]
        #(#other_attrs)*
        pub #usage fn #receiver_name(&self) -> #zbus::Result<#stream_name<'c>>
        {
            self.receive_signal(#signal_name)#wait.map(#stream_name)
        }

        #receive_signal_with_args
    };

    let stream_gen_doc = format!(
        "A [`{trait_name}`] implementation that yields [`signal_name`] signals.\n\
            \n\
            Use [`{proxy_name}::{receiver_name}`] to create an instance of this type.\n\
            \n\
            [`{trait_name}`]: {trait_link}",
    );
    let signal_args_gen_doc = format!("`{signal_name}` signal arguments.");
    let args_struct_gen_doc = format!("A `{signal_name}` signal.");
    let args_impl = {
        let arg_fields_init = if args.len() == 1 {
            quote! { #(#args)*: args }
        } else {
            quote! { #(#args: args.#args_nth),* }
        };

        quote! {
            impl #signal_name_ident {
                /// Retrieve the signal arguments.
                pub fn args#ty_generics(&'s self) -> #zbus::Result<#signal_args #ty_generics>
                #where_clause
                {
                    ::std::convert::TryFrom::try_from(&**self)
                }
            }

            #[doc = #signal_args_gen_doc]
            pub struct #signal_args #ty_generics {
                phantom: std::marker::PhantomData<&'s ()>,
                #(
                    pub #args: #input_types_s
                 ),*
            }

            impl #impl_generics #signal_args #ty_generics
                #where_clause
            {
                #(
                    pub fn #args(&self) -> &#input_types_s {
                        &self.#args
                    }
                 )*
            }

            impl #impl_generics std::fmt::Debug for #signal_args #ty_generics
                #where_clause
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(#signal_name)
                    #(
                     .field(stringify!(#args), &self.#args)
                    )*
                     .finish()
                }
            }

            impl #impl_generics ::std::convert::TryFrom<&'s #zbus::Message> for #signal_args #ty_generics
                #where_clause
            {
                type Error = #zbus::Error;

                fn try_from(message: &'s #zbus::Message) -> #zbus::Result<Self> {
                    message.body::<(#(#input_types),*)>()
                        .map_err(::std::convert::Into::into)
                        .map(|args| {
                            #signal_args {
                                phantom: ::std::marker::PhantomData,
                                #arg_fields_init
                            }
                        })
                }
            }
        }
    };

    receive_signal
}
