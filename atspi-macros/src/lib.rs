#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
#[cfg(feature = "unstable_atspi_proxy_macro")]
mod proxy;
#[cfg(feature = "unstable_atspi_proxy_macro")]
mod utils;
#[cfg(feature = "unstable_atspi_proxy_macro")]
mod zbus_proxy;

#[cfg(feature = "unstable_atspi_proxy_macro")]
use syn::ItemTrait;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse_macro_input, AttributeArgs, DeriveInput, ItemStruct, Lit, Meta, MetaNameValue,
	NestedMeta, Type,
};

use std::convert::TryFrom;

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

#[proc_macro_attribute]
#[cfg(feature = "unstable_atspi_proxy_macro")]
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
		let item = Accessible {
		  name: message
			.header()?
			.sender()?
			.ok_or(ObjectPathConversionError::NoIdAvailable)?
			.to_owned()
			.into(),
		  path: message
			.path()
			.ok_or(ObjectPathConversionError::NoIdAvailable)?
			.into()
		};
				let message_member: MemberName = message
					.member()
					.ok_or(AtspiError::MemberMatch("message w/o member".to_string()))?;

				let member = MemberName::from_static_str(#member)?;

				if message_member != member {
					let error = format!("message member: {:?} != member: {:?}", message_member, member);
					return Err(AtspiError::MemberMatch(error));
				};
				let body: #body_type = message.body()?;
				Ok(Self { item, body })
			}
		}
	};

	// Return the expanded code as a token stream
	TokenStream::from(expanded)
}
