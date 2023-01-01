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
    parse_macro_input, AttributeArgs, DeriveInput, ItemStruct, Lit, Meta, MetaNameValue,
		Ident,
    NestedMeta, Type,
};
use zbus::xml::*;

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
	Kind,
	Detail1,
	Detail2,
	AnyData,
	Properties,
}
impl ToString for AtspiEventInnerName {
	fn to_string(&self) -> String {
		match self {
			Self::Kind => "kind",
			Self::Detail1 => "detail1",
			Self::Detail2 => "detail2",
			Self::AnyData => "any_data",
			Self::Properties => "properties",
		}.to_string()
	}
}
impl From<usize> for AtspiEventInnerName {
	fn from(from: usize) -> AtspiEventInnerName {
		match from {
			0 => Self::Kind,
			1 => Self::Detail1,
			2 => Self::Detail2,
			3 => Self::AnyData,
			4 => Self::Properties,
			_ => panic!("Invalid AtspiEventInnerName usize value"),
		}
	}
}

fn generate_fn_handle_for_signal_item(signal_item: Arg, inner_event_name: AtspiEventInnerName) -> TokenStream2 {
	if signal_item.name().is_none() {
		return quote!{};
	}
	// unwrap is safe due to check
	let function_name = Ident::new(&signal_item.name().unwrap(), Span::call_site());
	let inner_name = Ident::new(&inner_event_name.to_string(), Span::call_site());
	
	quote!{}
}

fn generate_impl_from_signal(signal: Signal) -> TokenStream2 {
	let mut sig_name_event_str = String::from(signal.name());
	sig_name_event_str.push_str("Event");
	let sig_name_event = Ident::new(&sig_name_event_str, Span::call_site());
	quote!{}
}

fn generate_struct_from_signal(signal: Signal) -> TokenStream2 {
	let mut sig_name_event_str = String::from(signal.name());
	sig_name_event_str.push_str("Event");
	let sig_name_event = Ident::new(&sig_name_event_str, Span::call_site());
	quote! {
		#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
		pub struct #sig_name_event(pub(crate) AtspiEvent);
	}
}

fn generate_variant_from_signal(signal: &Signal) -> TokenStream2 {
	let sig_name = Ident::new(&signal.name(), Span::call_site());
	let mut sig_name_event_str = String::from(signal.name());
	sig_name_event_str.push_str("Event");
	let sig_name_event = Ident::new(&sig_name_event_str, Span::call_site());
	quote!{
		#sig_name(#sig_name_event),
	}
}

fn generate_enum_from_iface(iface: Interface) -> TokenStream2 {
	let mut name = iface.name().split('.').next_back().expect("Interface must contain a period").to_string();
	name.push_str("Events");
	let name_ident = Ident::new(&name, Span::call_site());
	let signal_quotes: Vec<TokenStream2> = iface.signals()
		.into_iter()
		.map(|signal| generate_variant_from_signal(signal))
		.collect();
	let signal_quote = TokenStream2::from_iter(signal_quotes.into_iter());
	quote! {
		pub enum #name_ident {
#signal_quote
		}
	}
}

//#[proc_macro_derive(TryFromMessage)]
#[proc_macro_attribute]
pub fn create_from_xml(attr: TokenStream, input: TokenStream) -> TokenStream {
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
	let mut mods_so_far = quote! {
	};
	mods_so_far.into()
}

//#[proc_macro_derive(TryFromMessage)]
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
