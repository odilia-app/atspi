use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use regex::Regex;
use std::{
	collections::HashMap,
	str::FromStr,
};
use syn::{
    self, fold::Fold, parse_quote, spanned::Spanned, Error, FnArg, Ident, ItemTrait, ReturnType, TraitItemMethod,
};

use crate::utils::*;

/// The name of an object pair. See: [`atspi::accessible::ObjectPair`].
const OBJECT_PAIR_NAME: &'static str = "ObjectPair";


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
			trait_name.to_string()
		};
		let trait_impl_name = TokenStream::from_str(&trait_impl_name_string).expect("Could not create token stream from \"{trait_impl_name_string}\"");
		let proxy_name = TokenStream::from_str(&proxy_name_string)?;
    let _other_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let _trait_name = Ident::new(trait_name, Span::call_site());
    let _ident = input.ident.to_string();
    let mut methods = TokenStream::new();
    let _trait_methods = TokenStream::new();
    let _stream_types = TokenStream::new();
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
            let _is_signal = attrs.iter().any(|x| x.is_signal());
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
            } else {
                gen_proxy_trait_method_impl(&member_name, &method_name, &proxy_name_string, m, &async_opts)
            };
            methods.extend(m);
        }
    }

    let AsyncOpts {   .. } = async_opts;
    let (_proxy_struct, _connection, _builder) = if blocking {
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

    let _other_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let trait_name = Ident::new(trait_name, Span::call_site());
    let _ident = input.ident.to_string();
    let _methods = TokenStream::new();
    let mut trait_methods = TokenStream::new();
    let _stream_types = TokenStream::new();
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
            let _is_signal = attrs.iter().any(|x| x.is_signal());
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

    let AsyncOpts {   .. } = async_opts;
    let (_proxy_struct, _connection, _builder) = if blocking {
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
				pub trait #trait_name {
					type Error: std::error::Error;
					#trait_methods
				}
		})
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
	let mut generic_impl = generic_result.replace(OBJECT_PAIR_NAME, "Self");
	generic_impl.push_str(" where Self: Sized");
	TokenStream::from_str(&generic_impl).expect("Could not genericize zbus method/property/signal. Attempted to turn \"{generic_result}\" into a TokenStream.")
}

fn gen_trait_method_signature(
    _method_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait: _,
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
    let _proxy_object = attrs.iter().find_map(|x| match x {
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

    let _method_flags = match (no_reply, no_autostart, allow_interactive_auth) {
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
    let (_, _ty_generics, _where_clause) = generics.split_for_impl();

		let _body = if args.len() == 1 {
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
    _method_name: &str,
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
    let _proxy_object = attrs.iter().find_map(|x| match x {
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

    let _method_flags = match (no_reply, no_autostart, allow_interactive_auth) {
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
    let (_, _ty_generics, _where_clause) = generics.split_for_impl();

		let body = if args.len() == 1 {
				// Wrap single arg in a tuple so if it's a struct/tuple itself, zbus will only remove
				// the '()' from the signature that we add and not the actual intended ones.
				let arg = &args[0];
				quote! {
						#arg
				}
		} else if args.is_empty() {
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
			// this is only one function: get_relation_set
		} else if output_str.contains("RelationType, Vec < Self") {
			quote! {
				#(#other_attrs)*
				#usage #signature {
					let raw_relation_sets = self.#method(#body)#wait?;
					let mut relation_sets = Vec::new();
					let conn = self.connection().clone();
					for raw_relation_set in raw_relation_sets {
						let mut proxies = Vec::new();
						for object_pair in raw_relation_set.1 {
							let proxy = #proxy::builder(&conn)
								.path(object_pair.1)?
								.destination(object_pair.0)?
								.build()
								#wait?;
							proxies.push(proxy);
						}
						relation_sets.push((raw_relation_set.0, proxies));
					}
					Ok(relation_sets)
				}
			}
		} else if output_str.contains("< Vec < Self") {
			quote! {
				#(#other_attrs)*
				#usage #signature {
					let vec_of_object_pairs = self.#method(#body)#wait?;
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
	} else if inputs.len() > 1 {
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
    _property_name: &str,
    method_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
    _emits_changed_signal: PropertyEmitsChangedSignal,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait: _,
        blocking: _,
    } = async_opts;
    let _zbus = zbus_path();
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("dbus_proxy"))
        .collect();
    let method = Ident::new(method_name, Span::call_site());
		let _signature = &m.sig;
    let inputs = &m.sig.inputs;
    let output = genericize_method_return_type(&m.sig.output);
		// do not process methods setting property values
		if inputs.len() > 1 {
			quote! {}
		} else {
			quote! {
					#(#other_attrs)*
					#usage fn #method(#inputs) #output;
			}
		}
}
fn gen_proxy_trait_impl_property(
    _property_name: &str,
    method_name: &str,
		proxy_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
    _emits_changed_signal: PropertyEmitsChangedSignal,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let zbus = zbus_path();
    let args: Vec<_> = m
        .sig
        .inputs
        .iter()
        .filter_map(typed_arg)
        .filter_map(pat_ident)
        .collect();
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
				// do not include property update method
        quote! {}
    } else {
        // This should fail to compile only if the return type is wrong,
        // so use that as the span.
        let _body_span = if let ReturnType::Type(_, ty) = &signature.output {
            ty.span()
        } else {
            signature.span()
        };
				let output_str = format!("{}", output);
				let proxy = TokenStream::from_str(proxy_name).expect("Could not create token stream from \"{proxy_name}\"");
				let input_args = if args.len() == 1 {
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
				} else if inputs.len() > 1 {
    						quote! {
    							self.#method(#input_args)#wait
    						}
    					} else {
    						quote! {
    							self.#method()#wait
    						}
    					};
        let _ret_type = if let ReturnType::Type(_, ty) = &signature.output {
            Some(ty)
        } else {
            None
        };

        let (_proxy_name, _prop_stream) = if *blocking {
            (
                "zbus::blocking::Proxy",
                quote! { #zbus::blocking::PropertyIterator },
            )
        } else {
            ("zbus::Proxy", quote! { #zbus::PropertyStream })
        };

				if !inputs.is_empty() {
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
