#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::{
	error::AtspiError,
	events::{
		BusProperties, EventWrapperMessageConversion, HasInterfaceName, HasMatchRule,
		HasRegistryEventString,
	},
	Event, EventProperties, EventTypeProperties,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

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
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl BusProperties for FocusEvent {
	const DBUS_MEMBER: &'static str = "Focus";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";
}

impl HasInterfaceName for FocusEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for FocusEvents {
	fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let header = msg.header();
		let member = header.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			"Focus" => Ok(FocusEvents::Focus(FocusEvent::try_from_message_unchecked(msg)?)),
			_ => Err(AtspiError::MemberMatch(format!(
				"No matching member {member} for interface {}",
				Self::DBUS_INTERFACE,
			))),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for FocusEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		let header = msg.header();
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		if interface != FocusEvents::DBUS_INTERFACE {
			return Err(AtspiError::InterfaceMatch(format!(
				"Interface {} does not match require interface for event: {}",
				interface,
				FocusEvents::DBUS_INTERFACE
			)));
		}
		Self::try_from_message_interface_checked(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(FocusEvent, FocusEvents, FocusEvents::Focus);
impl_from_user_facing_type_for_event_enum!(FocusEvent, Event::Focus);
impl_try_from_event_for_user_facing_type!(FocusEvent, FocusEvents::Focus, Event::Focus);

event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl_event_properties!(FocusEvent);
impl_from_object_ref!(FocusEvent);

impl HasRegistryEventString for FocusEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Focus:";
}
