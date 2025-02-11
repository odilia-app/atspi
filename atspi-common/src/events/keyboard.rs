#[cfg(feature = "zbus")]
use crate::{
	error::AtspiError,
	events::{MessageConversion, MessageConversionExt},
	ObjectRef,
};
use crate::{events::event_body::EventBodyOwned, EventProperties};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, OwnedValue};


use super::{event_body::Properties, BusProperties};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum KeyboardEvents {
	/// See: [`ModifiersEvent`].
	Modifiers(ModifiersEvent),
}

impl EventTypeProperties for KeyboardEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.member(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.match_rule(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.interface(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for KeyboardEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Modifiers(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Modifiers(inner) => inner.sender(),
		}
	}
}

impl_from_interface_event_enum_for_event!(KeyboardEvents, Event::Keyboard);
impl_try_from_event_for_user_facing_event_type!(KeyboardEvents, Event::Keyboard);

event_wrapper_test_cases!(KeyboardEvents, ModifiersEvent);

impl HasMatchRule for KeyboardEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Keyboard'";
}

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
impl MessageConversion for ModifiersEvent {
	type Body = EventBodyOwned;

	fn from_message_unchecked_parts(item: ObjectRef, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, previous_modifiers: body.detail1, current_modifiers: body.detail2 })
	}
	fn from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body();
		let body: Self::Body = body.deserialize_unchecked()?;
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

event_test_cases!(ModifiersEvent);
impl_to_dbus_message!(ModifiersEvent);
impl_from_dbus_message!(ModifiersEvent);
impl_event_properties!(ModifiersEvent);
impl From<ModifiersEvent> for EventBodyOwned {
	fn from(event: ModifiersEvent) -> Self {
		EventBodyOwned {
<<<<<<< HEAD
			properties: Properties,
||||||| parent of a290b37 (refactor(common): event_body.rs and refactor object_ref (squashed))
			properties: std::collections::HashMap::new(),
=======
>>>>>>> a290b37 (refactor(common): event_body.rs and refactor object_ref (squashed))
			kind: String::default(),
			detail1: event.previous_modifiers,
			detail2: event.current_modifiers,
			any_data: OwnedValue::from(0u8),
			properties: Properties,
		}
	}
}
