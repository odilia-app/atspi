#[cfg(feature = "zbus")]
use super::event_body::EventBody;
#[cfg(feature = "zbus")]
use crate::error::AtspiError;
use crate::{
	events::{DBusInterface, DBusMatchRule, DBusMember, EventBodyOwned, RegistryEventString},
	object_ref::NonNullObjectRef,
};

#[cfg(feature = "zbus")]
use crate::{events::MessageConversion, EventProperties};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ModifiersEvent<'o> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'o>,
	pub previous_modifiers: i32,
	pub current_modifiers: i32,
}

impl_event_type_properties_for_event!(ModifiersEvent<'_>);

impl_member_interface_registry_string_and_match_rule_for_event! {
	ModifiersEvent<'_>,
	"Modifiers",
	"org.a11y.atspi.Event.Keyboard",
	"keyboard:modifiers",
	"type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'"
}

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for ModifiersEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, previous_modifiers: body.detail1(), current_modifiers: body.detail2() })
	}

	fn from_message_unchecked(msg: &'a zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned {
			detail1: self.previous_modifiers,
			detail2: self.current_modifiers,
			..Default::default()
		}
		.into()
	}
}

impl_msg_conversion_ext_for_target_type!(ModifiersEvent<'_>);

event_test_cases!(ModifiersEvent);
impl_to_dbus_message!(ModifiersEvent<'_>);
impl_from_dbus_message!(ModifiersEvent<'_>);
impl_event_properties!(ModifiersEvent<'_>);
impl_test_event!(ModifiersEvent<'_> { previous_modifiers, current_modifiers });

impl From<ModifiersEvent<'_>> for EventBodyOwned {
	fn from(event: ModifiersEvent) -> Self {
		EventBodyOwned {
			detail1: event.previous_modifiers,
			detail2: event.current_modifiers,
			..Default::default()
		}
	}
}
