#[cfg(feature = "zbus")]
use crate::{
	error::AtspiError,
	events::{MessageConversion, MessageConversionExt},
	ObjectRef,
};
use crate::{
	events::{BusProperties, EventBodyOwned},
	EventProperties,
};
use zbus::message::{Body as DbusBody, Header};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

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

impl BusProperties for RelEvent {
	const DBUS_MEMBER: &'static str = "Rel";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
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

impl BusProperties for ButtonEvent {
	const DBUS_MEMBER: &'static str = "Button";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
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
