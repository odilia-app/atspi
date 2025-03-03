use super::event_body::EventBody;
use crate::{
	error::AtspiError,
	events::{
		BusProperties, EventBodyOwned, HasInterfaceName, HasMatchRule, HasRegistryEventString,
	},
	Event, EventProperties, EventTypeProperties,
};
#[cfg(feature = "zbus")]
use crate::{
	events::{
		EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
	},
	ObjectRef,
};
use zbus::message::{Body as DbusBody, Header};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

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

impl HasInterfaceName for KeyboardEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for KeyboardEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			ModifiersEvent::DBUS_MEMBER => {
				Ok(KeyboardEvents::Modifiers(ModifiersEvent::from_message_unchecked(msg)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for KeyboardEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	ModifiersEvent,
	KeyboardEvents,
	KeyboardEvents::Modifiers
);
impl_from_user_facing_type_for_event_enum!(ModifiersEvent, Event::Keyboard);
impl_try_from_event_for_user_facing_type!(
	ModifiersEvent,
	KeyboardEvents::Modifiers,
	Event::Keyboard
);

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

impl HasRegistryEventString for KeyboardEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";
}
