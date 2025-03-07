#[cfg(feature = "zbus")]
use crate::{
	error::AtspiError,
	events::{DBusInterface, DBusMatchRule, RegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
use zbus::message::Header;
use zbus_names::UniqueName;
use zvariant::ObjectPath;

use super::DBusMember;

impl_try_from_event_for_user_facing_event_type!(FocusEvents, Event::Focus);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct FocusEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl_member_interface_registry_string_and_match_rule_for_event! {
	FocusEvent,
	"Focus",
	"org.a11y.atspi.Event.Focus",
	"focus:",
	"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'"
}

impl_msg_conversion_ext_for_target_type!(FocusEvent);
impl_msg_conversion_for_types_built_from_object_ref!(FocusEvent);

event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl_event_properties!(FocusEvent);
impl_from_object_ref!(FocusEvent);
