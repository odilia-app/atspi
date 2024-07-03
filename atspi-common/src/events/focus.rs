#[cfg(feature = "zbus")]
use crate::events::{MessageConversion, ObjectRef};
use crate::{
	error::AtspiError,
	events::{BusProperties, EventBodyOwned, HasMatchRule, HasRegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
use zbus_names::UniqueName;
use zvariant::{ObjectPath, OwnedValue};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum FocusEvents {
	/// See: [`FocusEvent`].
	Focus(FocusEvent),
}

impl EventTypeProperties for FocusEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.member(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.match_rule(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.interface(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for FocusEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Focus(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Focus(inner) => inner.sender(),
		}
	}
}

impl_from_interface_event_enum_for_event!(FocusEvents, Event::Focus);
impl_try_from_event_for_user_facing_event_type!(FocusEvents, Event::Focus);

event_wrapper_test_cases!(FocusEvents, FocusEvent);

impl HasMatchRule for FocusEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct FocusEvent {
	/// The [`ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl BusProperties for FocusEvent {
	const DBUS_MEMBER: &'static str = "Focus";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";
}

#[cfg(feature = "zbus")]
impl MessageConversion for FocusEvent {
	type Body = EventBodyOwned;

	fn from_message_parts_unchecked(
		item: ObjectRef,
		_body: zbus::message::Body,
	) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for FocusEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let header = ev.header();
		let member = header
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"Focus" => Ok(FocusEvents::Focus(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(FocusEvent, FocusEvents, FocusEvents::Focus);
impl_from_user_facing_type_for_event_enum!(FocusEvent, Event::Focus);
impl_try_from_event_for_user_facing_type!(FocusEvent, FocusEvents::Focus, Event::Focus);

event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl_event_properties!(FocusEvent);
impl From<FocusEvent> for EventBodyOwned {
	fn from(_event: FocusEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: OwnedValue::from(0u8),
		}
	}
}

impl HasRegistryEventString for FocusEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";
}
