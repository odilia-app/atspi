use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
use zvariant::ObjectPath;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum KeyboardEvents {
	/// See: [`ModifiersEvent`].
	Modifiers(ModifiersEvent),
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
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	pub previous_modifiers: i32,
	pub current_modifiers: i32,
}

impl GenericEvent<'_> for ModifiersEvent {
	const DBUS_MEMBER: &'static str = "Modifiers";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
	const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, previous_modifiers: body.detail1, current_modifiers: body.detail2 })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for KeyboardEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"Modifiers" => Ok(KeyboardEvents::Modifiers(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
		}
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
impl From<ModifiersEvent> for EventBodyOwned {
	fn from(event: ModifiersEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.previous_modifiers,
			detail2: event.current_modifiers,
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl HasRegistryEventString for KeyboardEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";
}
