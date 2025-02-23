use super::event_body::{EventBody, Properties};
use crate::{
	error::AtspiError,
	events::{
		BusProperties, EventBodyOwned, HasInterfaceName, HasMatchRule, HasRegistryEventString,
	},
	Event, EventProperties, EventTypeProperties,
};

#[cfg(feature = "zbus")]
use crate::{
	error::AtspiError,
	events::{MessageConversion, MessageConversionExt},
	ObjectRef,
};
use crate::{events::event_body::EventBodyOwned, EventProperties};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, OwnedValue};

impl_try_from_event_for_user_facing_event_type!(KeyboardEvents, Event::Keyboard);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ModifiersEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
	pub previous_modifiers: i32,
	pub current_modifiers: i32,
}

impl BusProperties for ModifiersEvent {
	const DBUS_MEMBER: &'static str = "Modifiers";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
	const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for ModifiersEvent {
	type Body<'msg> = EventBody<'msg>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, previous_modifiers: body.detail1(), current_modifiers: body.detail2() })
	}

	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
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

event_test_cases!(ModifiersEvent);
impl_to_dbus_message!(ModifiersEvent);
impl_from_dbus_message!(ModifiersEvent);
impl_event_properties!(ModifiersEvent);
impl From<ModifiersEvent> for EventBodyOwned {
	fn from(event: ModifiersEvent) -> Self {
		EventBodyOwned {
			detail1: event.previous_modifiers,
			detail2: event.current_modifiers,
			..Default::default()
		}
	}
}
