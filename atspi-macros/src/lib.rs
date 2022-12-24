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
					fn path(&self) -> std::option::Option<zbus::zvariant::OwnedObjectPath> {
							Some(OwnedObjectPath::from(self.message.path().unwrap()))
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
