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
use syn::{parse_macro_input, DeriveInput, Type};

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
			0 | 4 => Err(ConversionError::FunctionAlreadyCreatedFor),
			1 => Ok(Self::Detail1),
			2 => Ok(Self::Detail2),
			3 => Ok(Self::AnyData),
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
