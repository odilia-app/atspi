#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::{
	error::AtspiError,
	events::{
		BusProperties, EventBodyOwned, HasInterfaceName, HasMatchRule, HasRegistryEventString,
	},
	Event, EventProperties, EventTypeProperties,
};
use zbus_names::UniqueName;
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

impl EventTypeProperties for MouseEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.member(),
			Self::Rel(inner) => inner.member(),
			Self::Button(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.interface(),
			Self::Rel(inner) => inner.interface(),
			Self::Button(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.match_rule(),
			Self::Rel(inner) => inner.match_rule(),
			Self::Button(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.registry_string(),
			Self::Rel(inner) => inner.registry_string(),
			Self::Button(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for MouseEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Abs(inner) => inner.path(),
			Self::Rel(inner) => inner.path(),
			Self::Button(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Abs(inner) => inner.sender(),
			Self::Rel(inner) => inner.sender(),
			Self::Button(inner) => inner.sender(),
		}
	}
}

impl_from_interface_event_enum_for_event!(MouseEvents, Event::Mouse);
impl_try_from_event_for_user_facing_event_type!(MouseEvents, Event::Mouse);

event_wrapper_test_cases!(MouseEvents, AbsEvent);

impl HasMatchRule for MouseEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AbsEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
	pub x: i32,
	pub y: i32,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RelEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
	pub x: i32,
	pub y: i32,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ButtonEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
	pub detail: String,
	pub mouse_x: i32,
	pub mouse_y: i32,
}

impl BusProperties for AbsEvent {
	const DBUS_MEMBER: &'static str = "Abs";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
}

#[cfg(feature = "zbus")]
impl MessageConversion for AbsEvent {
	type Body = EventBodyOwned;

	fn try_from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body();
		let ev_body: Self::Body = body.deserialize_unchecked()?;
		Ok(Self { item, x: ev_body.detail1, y: ev_body.detail2 })
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl BusProperties for RelEvent {
	const DBUS_MEMBER: &'static str = "Rel";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
}

#[cfg(feature = "zbus")]
impl MessageConversion for RelEvent {
	type Body = EventBodyOwned;

	fn try_from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body();
		let ev_body: Self::Body = body.deserialize_unchecked()?;
		Ok(Self { item, x: ev_body.detail1, y: ev_body.detail2 })
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl BusProperties for ButtonEvent {
	const DBUS_MEMBER: &'static str = "Button";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
}

#[cfg(feature = "zbus")]
impl MessageConversion for ButtonEvent {
	type Body = EventBodyOwned;

	fn try_from_message_unchecked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let item = msg.try_into()?;
		let body = msg.body();
		let ev_body: Self::Body = body.deserialize_unchecked()?;
		Ok(Self { item, detail: ev_body.kind, mouse_x: ev_body.detail1, mouse_y: ev_body.detail2 })
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}
impl HasInterfaceName for MouseEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for MouseEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		let header = msg.header();
		let member = header.member().ok_or(AtspiError::MissingMember)?;
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		if interface != MouseEvents::DBUS_INTERFACE {
			return Err(AtspiError::InterfaceMatch(format!(
				"Interface {} does not match required interface for event: {}",
				interface,
				MouseEvents::DBUS_INTERFACE
			)));
		}
		match member.as_str() {
			"Abs" => Ok(MouseEvents::Abs(AbsEvent::try_from_message_unchecked(msg)?)),
			"Rel" => Ok(MouseEvents::Rel(RelEvent::try_from_message_unchecked(msg)?)),
			"Button" => Ok(MouseEvents::Button(ButtonEvent::try_from_message_unchecked(msg)?)),
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
impl_event_properties!(AbsEvent);
impl From<AbsEvent> for EventBodyOwned {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.x,
			detail2: event.y,
			any_data: u8::default().into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(RelEvent, MouseEvents, MouseEvents::Rel);
impl_from_user_facing_type_for_event_enum!(RelEvent, Event::Mouse);
impl_try_from_event_for_user_facing_type!(RelEvent, MouseEvents::Rel, Event::Mouse);
event_test_cases!(RelEvent);
impl_to_dbus_message!(RelEvent);
impl_from_dbus_message!(RelEvent);
impl_event_properties!(RelEvent);
impl From<RelEvent> for EventBodyOwned {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.x,
			detail2: event.y,
			any_data: u8::default().into(),
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
impl_event_properties!(ButtonEvent);
impl From<ButtonEvent> for EventBodyOwned {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.detail,
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			any_data: u8::default().into(),
		}
	}
}

impl HasRegistryEventString for MouseEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
}
