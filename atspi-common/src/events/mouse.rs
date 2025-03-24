use crate::{
	error::AtspiError,
	events::{DBusInterface, DBusMatchRule, EventBody, EventBodyOwned, RegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
#[cfg(feature = "zbus")]
use crate::{
	events::{
		EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
	},
	ObjectRef,
};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

use super::DBusMember;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MouseEvents {
	/// See: [`AbsEvent`].
	Abs(AbsEvent),

	/// See: [`RelEvent`].
	Rel(RelEvent),

	/// See: [`ButtonEvent`].
	Button(ButtonEvent),
}

impl_tryfrommessage_for_event_wrapper!(MouseEvents);

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

impl DBusMatchRule for MouseEvents {
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

impl_member_interface_registry_string_and_match_rule_for_event! {
	AbsEvent,
	"Abs",
	"org.a11y.atspi.Event.Mouse",
	"mouse:abs",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'"
}

impl_member_interface_registry_string_and_match_rule_for_event! {
	RelEvent,
	"Rel",
	"org.a11y.atspi.Event.Mouse",
	"mouse:rel",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'"
}

impl_member_interface_registry_string_and_match_rule_for_event! {
	ButtonEvent,
	"Button",
	"org.a11y.atspi.Event.Mouse",
	"mouse:button",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'"
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for AbsEvent {
	type Body<'a> = EventBody<'a>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, x: body.detail1(), y: body.detail2() })
	}
	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned { detail1: self.x, detail2: self.y, ..Default::default() }.into()
	}
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for RelEvent {
	type Body<'a> = EventBody<'a>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, x: body.detail1(), y: body.detail2() })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned { detail1: self.x, detail2: self.y, ..Default::default() }.into()
	}
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for ButtonEvent {
	type Body<'a> = EventBody<'a>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self {
			item,
			detail: body.take_kind(),
			mouse_x: body.detail1(),
			mouse_y: body.detail2(),
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self).into()
	}
}

impl DBusInterface for MouseEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
}

impl RegistryEventString for MouseEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for MouseEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			AbsEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Abs(AbsEvent::from_message_unchecked(msg, hdr)?))
			}
			RelEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Rel(RelEvent::from_message_unchecked(msg, hdr)?))
			}
			ButtonEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Button(ButtonEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for MouseEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
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
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<&AbsEvent> for EventBodyOwned {
	fn from(event: &AbsEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<AbsEvent> for EventBody<'_> {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned::from(event).into()
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
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<&RelEvent> for EventBodyOwned {
	fn from(event: &RelEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<RelEvent> for EventBody<'_> {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned::from(event).into()
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
			kind: event.detail,
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			..Default::default()
		}
	}
}

impl From<ButtonEvent> for EventBody<'_> {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

impl From<&ButtonEvent> for EventBodyOwned {
	fn from(event: &ButtonEvent) -> Self {
		EventBodyOwned {
			kind: event.detail.clone(),
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			..Default::default()
		}
	}
}

impl_msg_conversion_ext_for_target_type!(AbsEvent);
impl_msg_conversion_ext_for_target_type!(RelEvent);
impl_msg_conversion_ext_for_target_type!(ButtonEvent);
