mod zbus_proxy;
mod proxy;
mod utils;

use proc_macro::{
	TokenStream,
};
use proc_macro2::{
	Span,
	TokenStream as TokenStream2,
};
use quote::{
	quote,
};
use syn::{
    parse_macro_input, AttributeArgs, DeriveInput, ItemStruct, ItemTrait, Lit, Meta, MetaNameValue,
		Ident,
    NestedMeta, Type,
};
use zbus::{
	xml::*,
};
use zvariant::{
	    Basic, ObjectPath, Signature, ARRAY_SIGNATURE_CHAR, DICT_ENTRY_SIG_END_CHAR,
			    DICT_ENTRY_SIG_START_CHAR, STRUCT_SIG_END_CHAR, STRUCT_SIG_START_CHAR, VARIANT_SIGNATURE_CHAR,
};

use std::{
	str::FromStr,
	convert::{
		TryFrom,
		TryInto,
	},
	iter::FromIterator,
};

enum FromZbusMessageParam {
    Invalid,
    Body(Type),
    Member(String),
}

impl From<(String, String)> for FromZbusMessageParam {
    fn from(items: (String, String)) -> Self {
        match (items.0.as_str(), items.1.as_str()) {
            ("body", tp) => Self::Body(
                syn::parse_str(tp)
                    .expect("The value given to the 'body' parameter must be a valid type."),
            ),
            ("member", mem) => Self::Member(mem.to_string()),
            _ => Self::Invalid,
        }
    }
}

enum XmlGenParams {
	Invalid,
	FileName(String),
}
impl From<(String, String)> for XmlGenParams {
    fn from(items: (String, String)) -> Self {
        match (items.0.as_str(), items.1.as_str()) {
            ("filename", name) => Self::FileName(name.to_string()),
            _ => Self::Invalid,
        }
    }
}

//
// Derive macro for that implements TryFrom<Event> on a per name / member basis.
//

#[proc_macro_derive(TrySignify)]
pub fn implement_signified(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    // Extract the name of the struct
    let name = &ident;

    // Generate the expanded code
    let expanded = quote! {
        impl Signified for #name {
            type Inner = AtspiEvent;
            fn inner(&self) -> &Self::Inner {
                &self.0
            }

            /// Returns `properties`.
            fn properties(&self) -> &std::collections::HashMap<String, OwnedValue> {
                self.0.properties()
            }

            /// Returns `kind` body member.
            fn kind(&self) -> &str {
                self.inner().kind()
            }
        }
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

fn make_into_params<T>(items: AttributeArgs) -> Vec<T> 
	where T: From<(String, String)> {
    items
        .into_iter()
        .filter_map(|nm| match nm {
            // Only select certain tokens
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                eq_token: _,
                lit: Lit::Str(lstr),
            })) => Some(
                // Convert the segment of the path to a string
                (
                    path.segments
                        .into_iter()
                        .map(|seg| seg.ident.to_string())
                        .collect::<Vec<String>>()
                        .swap_remove(0),
                    // get the raw value of the LitStr
                    lstr.value(),
                ),
            ),
            _ => None,
        })
        // convert the (String, LitStr) tuple to a custom type which only accepts certain key/value pairs
        .map(|(k, v)| T::from((k, v)))
        .collect()
}

enum AtspiEventInnerName {
	Detail1,
	Detail2,
	AnyData,
}
impl ToString for AtspiEventInnerName {
	fn to_string(&self) -> String {
		match self {
			Self::Detail1 => "detail1",
			Self::Detail2 => "detail2",
			Self::AnyData => "any_data",
		}.to_string()
	}
}
enum ConversionError {
	FunctionAlreadyCreatedFor,
	UnknownItem,
}
impl TryFrom<usize> for AtspiEventInnerName {
	type Error = ConversionError;

	fn try_from(from: usize) -> Result<Self, Self::Error> {
		match from {
			0 => Err(ConversionError::FunctionAlreadyCreatedFor),
			1 => Ok(Self::Detail1),
			2 => Ok(Self::Detail2),
			3 => Ok(Self::AnyData),
			4 => Err(ConversionError::FunctionAlreadyCreatedFor),
			_ => Err(ConversionError::UnknownItem),
		}
	}
}

// taken from zbus_xmlgen: https://gitlab.freedesktop.org/dbus/zbus/-/blob/main/zbus_xmlgen/src/gen.rs
fn to_rust_type(ty: &str, input: bool, as_ref: bool) -> String {
    // can't haz recursive closure, yet
    fn iter_to_rust_type(
        it: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        input: bool,
        as_ref: bool,
    ) -> String {
        let c = it.next().unwrap();
        match *c as char {
            u8::SIGNATURE_CHAR => "u8".into(),
            bool::SIGNATURE_CHAR => "bool".into(),
            i16::SIGNATURE_CHAR => "i16".into(),
            u16::SIGNATURE_CHAR => "u16".into(),
            i32::SIGNATURE_CHAR => "i32".into(),
            u32::SIGNATURE_CHAR => "u32".into(),
            i64::SIGNATURE_CHAR => "i64".into(),
            u64::SIGNATURE_CHAR => "u64".into(),
            f64::SIGNATURE_CHAR => "f64".into(),
            // xmlgen accepts 'h' on Windows, only for code generation
            'h' => (if input {
                "zbus::zvariant::Fd"
            } else {
                "zbus::zvariant::OwnedFd"
            })
            .into(),
            <&str>::SIGNATURE_CHAR => (if input || as_ref { "&str" } else { "String" }).into(),
            ObjectPath::SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::ObjectPath<'_>"
                } else {
                    "zbus::zvariant::ObjectPath<'_>"
                }
            } else {
                "zbus::zvariant::OwnedObjectPath"
            })
            .into(),
            Signature::SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::Signature<'_>"
                } else {
                    "zbus::zvariant::Signature<'_>"
                }
            } else {
                "zbus::zvariant::OwnedSignature"
            })
            .into(),
            VARIANT_SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::Value<'_>"
                } else {
                    "zbus::zvariant::Value<'_>"
                }
            } else {
                "zbus::zvariant::OwnedValue"
            })
            .into(),
            ARRAY_SIGNATURE_CHAR => {
                let c = it.peek().unwrap();
                match **c as char {
                    '{' => format!(
                        "std::collections::HashMap<{}>",
                        iter_to_rust_type(it, input, false)
                    ),
                    _ => {
                        let ty = iter_to_rust_type(it, input, false);
                        if input {
                            format!("&[{ty}]")
                        } else {
                            format!("{}Vec<{}>", if as_ref { "&" } else { "" }, ty)
                        }
                    }
                }
            }
            c @ STRUCT_SIG_START_CHAR | c @ DICT_ENTRY_SIG_START_CHAR => {
                let dict = c == '{';
                let mut vec = vec![];
                loop {
                    let c = it.peek().unwrap();
                    match **c as char {
                        STRUCT_SIG_END_CHAR | DICT_ENTRY_SIG_END_CHAR => break,
                        _ => vec.push(iter_to_rust_type(it, input, false)),
                    }
                }
                if dict {
                    vec.join(", ")
                } else if vec.len() > 1 {
                    format!("{}({})", if as_ref { "&" } else { "" }, vec.join(", "))
                } else {
                    vec[0].to_string()
                }
            }
            _ => unimplemented!(),
        }
    }

    let mut it = ty.as_bytes().iter().peekable();
    iter_to_rust_type(&mut it, input, as_ref)
}

// accept String or &str
fn str_ident<S>(string: S) -> Ident 
	where S: Into<String> {
	Ident::new(&string.into(), Span::call_site())
}

fn iface_name(iface: &Interface) -> String {
	iface.name().split('.').next_back().expect("An interface must have a period in its name.").to_string()
}

fn events_ident<S>(string: S) -> Ident 
	where S: Into<String> {
	let mut sig_name_event_str = string.into();
	sig_name_event_str.push_str("Events");
	str_ident(sig_name_event_str)
}
fn event_ident<S>(string: S) -> Ident 
	where S: Into<String> {
	let mut sig_name_event_str = string.into();
	sig_name_event_str.push_str("Event");
	str_ident(sig_name_event_str)
}

fn str_to_type<S>(s: S) -> Type 
	where S: Into<String> + Clone + std::fmt::Display {
	let rust_type_ts = TokenStream::from_str(&s.clone().into()).expect("The string \"{rust_type_str}\" is not able to be turned into a TokenStream");
	match parse_macro_input::parse::<Type>(rust_type_ts) {
		Ok(data) => data,
		_ => panic!("The string {} could not be converted to Type", s.clone()),
	}
}

fn generate_fn_for_signal_item(signal_item: &Arg, inner_event_name: AtspiEventInnerName) -> TokenStream2 {
	if signal_item.name().is_none() {
		return quote!{};
	}
	// unwrap is safe due to check
	let function_name = str_ident(signal_item.name().unwrap());
	let inner_name = str_ident(inner_event_name.to_string());
	let rust_type = str_to_type(to_rust_type(signal_item.ty(), true, true));
	
	quote!{
		#[must_use]
		pub fn #function_name(&self) -> #rust_type {
			self.0.#inner_name()
		}
	}
}

fn generate_impl_from_signal(signal: &Signal) -> TokenStream2 {
	let sig_name_event = event_ident(signal.name());
	let functions = TokenStream2::from_iter(
		signal.args()
			.iter()
			.enumerate()
				.filter_map(|(i, arg)| match i.try_into() {
					Ok(func_name) => Some(generate_fn_for_signal_item(arg, func_name)),
					Err(_) => None,
			})
	);
	quote!{
		impl #sig_name_event {
			#functions
		}
	}
}

fn generate_struct_from_signal(signal: &Signal) -> TokenStream2 {
	let sig_name_event = event_ident(signal.name());
	quote! {
		#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
		pub struct #sig_name_event(pub(crate) AtspiEvent);
	}
}

fn generate_variant_from_signal(signal: &Signal) -> TokenStream2 {
	let sig_name = str_ident(signal.name());
	let sig_name_event = event_ident(signal.name());
	quote!{
		#sig_name(#sig_name_event),
	}
}

fn match_arm_for_signal(iface_name: &str, signal: &Signal) -> TokenStream2 {
	let signal_name = signal.name();
	let enum_name = events_ident(iface_name);
	let signal_variant = str_ident(signal_name);
	let signal_struct_name = event_ident(signal_name);
	quote! {
		#signal_name => Ok(#enum_name::#signal_variant(#signal_struct_name(ev))),
	}
}

fn generate_try_from_atspi_event(iface: &Interface) -> TokenStream2 {
	let iname = iface_name(iface);
	let error_str = format!("No matching member for {iname}");
	let impl_for_name = events_ident(&iname);
	let member_conversions = TokenStream2::from_iter(
		iface.signals()
			.iter()
			.map(|signal| match_arm_for_signal(&iname, signal))
	);
	quote! {
		impl TryFrom<AtspiEvent> for #impl_for_name {
			type Error = AtspiError;

			fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
				let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
				match member.as_str() {
					#member_conversions
					_ => Err(AtspiError::MemberMatch(#error_str.into())),
				}
			}
		}
	}
}

fn generate_mod_from_iface(iface: &Interface) -> TokenStream2 {
	let mod_name = str_ident(iface_name(iface).to_lowercase());
	let enums = generate_enum_from_iface(iface);
	let structs = TokenStream2::from_iter(
		iface.signals()
			.iter()
			.map(|signal| generate_struct_from_signal(signal)));
	let impls = TokenStream2::from_iter(
		iface.signals()
			.iter()
			.map(|signal| generate_impl_from_signal(signal)));
	let try_froms = generate_try_from_atspi_event(iface);
	quote! {
		pub mod #mod_name {
			use atspi_macros::TrySignify;
			use crate::{
				error::AtspiError,
				events::{AtspiEvent, GenericEvent},
				identify::Signified,
			};
			use zbus;
			use zbus::zvariant::{OwnedObjectPath, OwnedValue};
			#enums
			#structs
			#impls
			#try_froms
		}
	}
}

fn generate_enum_from_iface(iface: &Interface) -> TokenStream2 {
	let name_ident = iface.name().split('.').next_back().expect("Interface must contain a period");
	let name_ident_plural = events_ident(name_ident);
	let signal_quotes = TokenStream2::from_iter(
		iface.signals()
			.into_iter()
			.map(|signal| generate_variant_from_signal(signal))
	);
	quote! {
		#[derive(Clone, Debug)]
		pub enum #name_ident_plural {
			#signal_quotes
		}
	}
}

//#[proc_macro_derive(TryFromMessage)]
#[proc_macro_attribute]
pub fn create_from_xml(attr: TokenStream, _input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(attr as AttributeArgs);
	let args_parsed: Vec<XmlGenParams> = make_into_params(args);
	let file_name = match args_parsed
			.get(0)
			.expect("There must be at least one argument to the macro.")
	{
			XmlGenParams::FileName(name) => name,
			_ => panic!("The file parameter must be set first, and must be a string."),
	};
	let xml_file = std::fs::File::open(file_name).expect("Cannot read file");
	let data: zbus::xml::Node = zbus::xml::Node::from_reader(&xml_file).expect("Cannot deserialize file");
	TokenStream2::from_iter(
		data.interfaces()
			.iter()
			.map(|iface| generate_mod_from_iface(iface))
	).into()
}

#[proc_macro_attribute]
pub fn atspi_proxy(attr: TokenStream, item: TokenStream) -> TokenStream {
	let args = parse_macro_input!(attr as AttributeArgs);
	let input = parse_macro_input!(item as ItemTrait);
	let zbus_part = zbus_proxy::expand(args, input.clone())
		.unwrap_or_else(|err| err.into_compile_error());
	let atspi_part = proxy::expand(input)
		.unwrap_or_else(|err| err.into_compile_error());
	quote! {
#zbus_part
#atspi_part
	}.into()
}

#[proc_macro_attribute]
pub fn try_from_zbus_message(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    // Parse the input token stream into a syntax tree
    let name = item_struct.ident.clone();

    // Remove the suffix "Event" from the name of the struct
    let name_string = name.to_string();

    let args = parse_macro_input!(attr as AttributeArgs);
		let args_parsed = make_into_params(args);
    let body_type = match args_parsed
        .get(0)
        .expect("There must be at least one argument to the macro.")
    {
        FromZbusMessageParam::Body(body_type) => body_type,
        _ => panic!("The body parameter must be set first, and must be a type."),
    };
    // if the member is set explicitly, use it, otherwise, use the struct name.
    let member = match args_parsed.get(1) {
        Some(FromZbusMessageParam::Member(member_str)) => member_str,
        _ => name_string.strip_suffix("Event").unwrap(),
    };

    // Generate the expanded code
    let expanded = quote! {
        #item_struct
        impl TryFrom<Arc<Message>> for  #name {
            type Error = AtspiError;

            fn try_from(message: Arc<Message>) -> Result<Self, Self::Error> {
                let message_member: MemberName = message
                    .member()
                    .ok_or(AtspiError::MemberMatch("message w/o member".to_string()))?;

                let member = MemberName::from_static_str(#member)?;

                if message_member != member {
                    let error = format!("message member: {:?} != member: {:?}", message_member, member);
                    return Err(AtspiError::MemberMatch(error));
                };
                let body: #body_type = message.body()?;
                Ok(Self { message, body })
            }
        }

    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}

#[proc_macro_derive(GenericEvent)]
pub fn generic_event(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    // Extract the name of the struct
    let name = &ident;

    // Generate the expanded code
    let expanded = quote! {
            impl GenericEvent for #name {
                    /// Bus message.
                    #[must_use]
                    fn message(&self) -> &Arc<Message> {
                            &self.message
                    }

                    /// For now this returns the full interface name because the lifetimes in [`zbus_names`][zbus::names] are
                    /// wrong such that the `&str` you can get from a
                    /// [`zbus_names::InterfaceName`][zbus::names::InterfaceName] is tied to the lifetime of that
                    /// name, not to the lifetime of the message as it should be. In future, this will return only
                    /// the last component of the interface name (I.E. "Object" from
                    /// "org.a11y.atspi.Event.Object").
                    #[must_use]
                    fn interface(&self) -> Option<InterfaceName<'_>> {
                            self.message.interface()
                    }

                    /// Identifies this event's interface member name.
                    #[must_use]
                    fn member(&self) -> Option<MemberName<'_>> {
                            self.message.member()
                    }

                    /// The object path to the object where the signal is emitted from.
                    #[must_use]
                    fn path(&self) -> std::option::Option<zbus::zvariant::ObjectPath<'_>> {
                            self.message.path()
                    }

                    /// Identifies the `sender` of the event.
                    /// # Errors
                    /// - when deserializeing the header failed, or
                    /// - When `zbus::get_field!` finds that 'sender' is an invalid field.
                    fn sender(&self) -> Result<Option<zbus::names::UniqueName>, crate::AtspiError> {
                            Ok(self.message.header()?.sender()?.cloned())
                    }
                }
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}
