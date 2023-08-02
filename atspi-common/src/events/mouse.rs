use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
use zvariant::ObjectPath;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MouseEvents {
	/// See: [`AbsEvent`].
	Abs(AbsEvent),
	/// See: [`RelEvent`].
	Rel(RelEvent),
	/// See: [`ButtonEvent`].
	Button(ButtonEvent),
}

impl_from_interface_event_enum_for_event!(MouseEvents, Event::Mouse);
impl_try_from_event_for_user_facing_event_type!(MouseEvents, Event::Mouse);

event_wrapper_test_cases!(MouseEvents, AbsEvent);

impl HasMatchRule for MouseEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AbsEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	pub x: i32,
	pub y: i32,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RelEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	pub x: i32,
	pub y: i32,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ButtonEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	pub detail: String,
	pub mouse_x: i32,
	pub mouse_y: i32,
}

impl GenericEvent<'_> for AbsEvent {
	const DBUS_MEMBER: &'static str = "Abs";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, x: body.detail1, y: body.detail2 })
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

impl GenericEvent<'_> for RelEvent {
	const DBUS_MEMBER: &'static str = "Rel";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, x: body.detail1, y: body.detail2 })
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

impl GenericEvent<'_> for ButtonEvent {
	const DBUS_MEMBER: &'static str = "Button";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, detail: body.kind, mouse_x: body.detail1, mouse_y: body.detail2 })
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
impl TryFrom<&zbus::Message> for MouseEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"Abs" => Ok(MouseEvents::Abs(ev.try_into()?)),
			"Rel" => Ok(MouseEvents::Rel(ev.try_into()?)),
			"Button" => Ok(MouseEvents::Button(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(AbsEvent, MouseEvents, MouseEvents::Abs);
impl_from_user_facing_type_for_event_enum!(AbsEvent, Event::Mouse);
impl_try_from_event_for_user_facing_type!(AbsEvent, MouseEvents::Abs, Event::Mouse);

event_test_cases!(AbsEvent);
impl_to_dbus_message!(AbsEvent);
impl_from_dbus_message!(AbsEvent);
impl From<AbsEvent> for EventBodyOwned {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.x,
			detail2: event.y,
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(RelEvent, MouseEvents, MouseEvents::Rel);
impl_from_user_facing_type_for_event_enum!(RelEvent, Event::Mouse);
impl_try_from_event_for_user_facing_type!(RelEvent, MouseEvents::Rel, Event::Mouse);
event_test_cases!(RelEvent);
impl_to_dbus_message!(RelEvent);
impl_from_dbus_message!(RelEvent);
impl From<RelEvent> for EventBodyOwned {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.x,
			detail2: event.y,
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	ButtonEvent,
	MouseEvents,
	MouseEvents::Button
);
impl_from_user_facing_type_for_event_enum!(ButtonEvent, Event::Mouse);
impl_try_from_event_for_user_facing_type!(ButtonEvent, MouseEvents::Button, Event::Mouse);
event_test_cases!(ButtonEvent);
impl_to_dbus_message!(ButtonEvent);
impl_from_dbus_message!(ButtonEvent);
impl From<ButtonEvent> for EventBodyOwned {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.detail,
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl HasRegistryEventString for MouseEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
}
