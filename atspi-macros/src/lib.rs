use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

//
// Derive macro for that implements TryFrom<Event> on a per name / member basis.
//

#[proc_macro_derive(TrySignify)]
pub fn try_signify(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    // Extract the name of the struct
    let name = &ident;

    // Remove the suffix "Event" from the name of the struct
    let name_string = name.to_string();
    let member = name_string.strip_suffix("Event").unwrap();

    // Generate the expanded code
    let expanded = quote! {
        impl TryFrom<AtspiEvent> for #name {
            type Error = crate::AtspiError;

            fn try_from(msg: AtspiEvent) -> Result<Self, Self::Error> {
                let msg_member = msg.message.member();
                if msg_member == Some(MemberName::from_static_str(#member)?) {
                    return Ok(Self(msg));
                };

                let tname = std::any::type_name::<Self>().to_string();
                let member = tname.strip_suffix("Event").unwrap();
                let error = format!("specific type's member: {} != msg type member: {:?}", member, msg_member);
                Err(crate::AtspiError::MemberMatch(error))
            }
        }

        impl<'a> Signified for #name {
            type Inner = AtspiEvent;
            fn inner(&self) -> &Self::Inner {
                &self.0
            }

            fn properties(&self) -> &HashMap<String, OwnedValue> {
                self.0.properties()
            }
        }
    };

    // Return the expanded code as a token stream
    TokenStream::from(expanded)
}
