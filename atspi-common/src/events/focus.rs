#[cfg(feature = "zbus")]
use crate::error::AtspiError;
#[cfg(feature = "zbus")]
use crate::EventProperties;
use crate::{
	events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString},
	object_ref::NonNullObjectRef,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct FocusEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(FocusEvent<'_>);

impl_member_interface_registry_string_and_match_rule_for_event! {
	FocusEvent<'_>,
	"Focus",
	"org.a11y.atspi.Event.Focus",
	"focus:",
	"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'"
}

impl_msg_conversion_ext_for_target_type!(FocusEvent<'_>);
impl_msg_conversion_for_types_built_from_object_ref!(FocusEvent<'_>);

event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent<'_>);
impl_from_dbus_message!(FocusEvent<'_>);
impl_event_properties!(FocusEvent<'_>);
impl_from_non_null_object_ref!(FocusEvent<'_>);
impl_test_event!(FocusEvent<'_>);
