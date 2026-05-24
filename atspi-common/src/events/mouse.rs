#[cfg(feature = "zbus")]
use crate::error::AtspiError;
#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::events::{
	DBusInterface, DBusMatchRule, DBusMember, EventBody, EventBodyOwned, RegistryEventString,
};
use crate::object_ref::NonNullObjectRef;
#[cfg(feature = "zbus")]
use crate::EventProperties;
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct AbsEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,

	/// x-axis coordinate as a distance from the root of the screen.\
	/// Usually root is the top-left of the screen.
	pub x: i32,
	/// y-axis coordinate as a distance from the root of the screen.\
	/// Usually root is the top-left of the screen.
	pub y: i32,
}

impl_event_type_properties_for_event!(AbsEvent<'_>);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct RelEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,

	/// x-axis coordinate as a distance from the window origin.\
	pub x: i32,
	/// y-axis coordinate as a distance from the root of the screen.\
	pub y: i32,
}

impl_event_type_properties_for_event!(RelEvent<'_>);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ButtonEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,

	/// Describes the button involved in the event.
	/// # Examples
	/// "p1": Button 1 pressed\
	/// "r1": Button 1 released
	pub detail: String,

	/// The absolute x-coordinate of the mouse pointer.
	pub mouse_x: i32,
	/// The absolute y-coordinate of the mouse pointer.
	pub mouse_y: i32,
}

impl_event_type_properties_for_event!(ButtonEvent<'_>);

impl_member_interface_registry_string_and_match_rule_for_event! {
	AbsEvent<'_>,
	"Abs",
	"org.a11y.atspi.Event.Mouse",
	"mouse:abs",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'"
}

impl_member_interface_registry_string_and_match_rule_for_event! {
	RelEvent<'_>,
	"Rel",
	"org.a11y.atspi.Event.Mouse",
	"mouse:rel",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'"
}

impl_member_interface_registry_string_and_match_rule_for_event! {
	ButtonEvent<'_>,
	"Button",
	"org.a11y.atspi.Event.Mouse",
	"mouse:button",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'"
}

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for AbsEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		// A short-lived amonymous borrow on `Body` is just fine as 'detail1' and 'detail2' are `Copy` types.
		// Meaning `Self` is not borrowing from `Body`.
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, x: body.detail1(), y: body.detail2() })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned { detail1: self.x, detail2: self.y, ..Default::default() }.into()
	}
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for RelEvent<'_> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		// A short-lived amonymous borrow on `Body` is just fine as 'detail1' and 'detail2' are `Copy` types.
		// Meaning `Self` is not borrowing from `Body`.
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item: item.into_owned(), x: body.detail1(), y: body.detail2() })
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
impl<'a> MessageConversion<'a> for ButtonEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		// In this case however we need to take ownership of 'kind'.
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self {
			item,
			detail: body.take_kind(),
			mouse_x: body.detail1(),
			mouse_y: body.detail2(),
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self).into()
	}
}

event_test_cases!(AbsEvent, [x, y], Auto);

impl_to_dbus_message!(AbsEvent<'_>);
impl_from_dbus_message!(AbsEvent<'_>);
impl_event_properties!(AbsEvent<'_>);

impl From<AbsEvent<'_>> for EventBodyOwned {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<&AbsEvent<'_>> for EventBodyOwned {
	fn from(event: &AbsEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<AbsEvent<'_>> for EventBody<'_> {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

event_test_cases!(RelEvent, [x, y], Auto);
impl_to_dbus_message!(RelEvent<'_>);
impl_from_dbus_message!(RelEvent<'_>);
impl_event_properties!(RelEvent<'_>);

impl From<RelEvent<'_>> for EventBodyOwned {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<&RelEvent<'_>> for EventBodyOwned {
	fn from(event: &RelEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<RelEvent<'_>> for EventBody<'_> {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

event_test_cases!(ButtonEvent, [detail, mouse_x, mouse_y], Auto);
impl_to_dbus_message!(ButtonEvent<'_>);
impl_from_dbus_message!(ButtonEvent<'_>);

impl_event_properties!(ButtonEvent<'_>);
impl From<ButtonEvent<'_>> for EventBodyOwned {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned {
			kind: event.detail,
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			..Default::default()
		}
	}
}

impl From<ButtonEvent<'_>> for EventBody<'_> {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

impl From<&ButtonEvent<'_>> for EventBodyOwned {
	fn from(event: &ButtonEvent) -> Self {
		EventBodyOwned {
			kind: event.detail.clone(),
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			..Default::default()
		}
	}
}

impl_msg_conversion_ext_for_target_type!(AbsEvent<'_>, RelEvent<'_>, ButtonEvent<'_>);

impl_test_event!(
	AbsEvent<'_> {x, y},
	RelEvent<'_> {x, y},
	ButtonEvent<'_> {detail, mouse_x, mouse_y}
);
