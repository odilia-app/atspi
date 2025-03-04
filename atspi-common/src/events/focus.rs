#[cfg(feature = "zbus")]
use crate::{
	error::AtspiError,
	events::{MessageConversion, MessageConversionExt},
};
use zbus::message::Header;
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

impl_msg_conversion_ext_for_target_type!(FocusEvent);
impl_msg_conversion_for_types_built_from_object_ref!(FocusEvent);

impl_try_from_event_for_user_facing_type!(FocusEvent, FocusEvents::Focus, Event::Focus);

event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl_event_properties!(FocusEvent);
impl_from_object_ref!(FocusEvent);
