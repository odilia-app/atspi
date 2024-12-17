#[cfg(feature = "zbus")]
use crate::events::{MessageConversion, MessageConversionExt};
use crate::{error::AtspiError, events::BusProperties, Event, EventProperties};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

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

impl_from_user_facing_type_for_event_enum!(FocusEvent, Event::Focus);

event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl_event_properties!(FocusEvent);
impl_from_object_ref!(FocusEvent);
