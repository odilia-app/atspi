#[cfg(feature = "unstable_atspi_proxy_codegen")]
mod proxy;
#[cfg(feature = "unstable_atspi_proxy_codegen")]
mod utils;
#[cfg(feature = "unstable_atspi_proxy_codegen")]
mod zbus_proxy;

#[cfg(feature = "unstable_atspi_proxy_codegen")]
use syn::ItemTrait;

#[cfg(feature = "unstable_atspi_proxy_codegen")]
use zbus::xml::*;

#[cfg(feature = "unstable_atspi_proxy_codegen")]
use zvariant::{
    Basic, ObjectPath, Signature, ARRAY_SIGNATURE_CHAR, DICT_ENTRY_SIG_END_CHAR,
    DICT_ENTRY_SIG_START_CHAR, STRUCT_SIG_END_CHAR, STRUCT_SIG_START_CHAR, VARIANT_SIGNATURE_CHAR,
};

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, DeriveInput, Ident, ItemStruct, Lit, Meta,
    MetaNameValue, NestedMeta, Type,
};

use std::{
    convert::{TryFrom, TryInto},
    iter::FromIterator,
    str::FromStr,
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
where
    T: From<(String, String)>,
{
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
        }
        .to_string()
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
#[cfg(feature = "unstable_atspi_proxy_codegen")]
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
            'h' => (if input { "zbus::zvariant::Fd" } else { "zbus::zvariant::OwnedFd" }).into(),
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

#[proc_macro_attribute]
#[cfg(feature = "unstable_atspi_proxy_codegen")]
pub fn atspi_proxy(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemTrait);
    let zbus_part =
        zbus_proxy::expand(args, input.clone()).unwrap_or_else(|err| err.into_compile_error());
    let atspi_part = proxy::expand(input).unwrap_or_else(|err| err.into_compile_error());
    quote! {
    #zbus_part
    #atspi_part
        }
    .into()
}
/// When the `traits` feature is not enabled, we alias `atspi_proxy` to [`zbus_macros::dbus_proxy`].
#[proc_macro_attribute]
#[cfg(not(feature = "unstable_atspi_proxy_codegen"))]
pub fn atspi_proxy(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = TokenStream2::from_str(&format!("#[zbus::dbus_proxy({})]", attr)).unwrap();
    let trait_fns = TokenStream2::from_str(&format!("{}", item)).unwrap();
		quote! {
#args
#trait_fns
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
