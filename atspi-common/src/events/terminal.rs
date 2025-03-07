use super::{DBusMatchRule, DBusMember};
#[cfg(feature = "zbus")]
use crate::{
	error::AtspiError,
	events::{DBusInterface, RegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
use zbus::message::Header;
use zbus_names::UniqueName;
use zvariant::ObjectPath;

impl_try_from_event_for_user_facing_event_type!(TerminalEvents, Event::Terminal);

/// A line of text has been changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnCountChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The height of a terminal emulator has changed sufficiently such that the number of lines
/// able to fit within the terminal has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LineCountChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ApplicationChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CharWidthChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	LineChangedEvent,
	"LineChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:line-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ColumnCountChangedEvent,
	"ColumncountChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:columncount-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LineCountChangedEvent,
	"LinecountChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:linecount-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ApplicationChangedEvent,
	"ApplicationChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:application-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	CharWidthChangedEvent,
	"CharwidthChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:char-width-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'"
);

event_test_cases!(LineChangedEvent);
impl_to_dbus_message!(LineChangedEvent);
impl_from_dbus_message!(LineChangedEvent);
impl_event_properties!(LineChangedEvent);
impl_from_object_ref!(LineChangedEvent);

event_test_cases!(ColumnCountChangedEvent);
impl_to_dbus_message!(ColumnCountChangedEvent);
impl_from_dbus_message!(ColumnCountChangedEvent);
impl_event_properties!(ColumnCountChangedEvent);
impl_from_object_ref!(ColumnCountChangedEvent);

event_test_cases!(LineCountChangedEvent);
impl_to_dbus_message!(LineCountChangedEvent);
impl_from_dbus_message!(LineCountChangedEvent);
impl_event_properties!(LineCountChangedEvent);
impl_from_object_ref!(LineCountChangedEvent);

event_test_cases!(ApplicationChangedEvent);
impl_to_dbus_message!(ApplicationChangedEvent);
impl_from_dbus_message!(ApplicationChangedEvent);
impl_event_properties!(ApplicationChangedEvent);
impl_from_object_ref!(ApplicationChangedEvent);

event_test_cases!(CharWidthChangedEvent);
impl_to_dbus_message!(CharWidthChangedEvent);
impl_from_dbus_message!(CharWidthChangedEvent);
impl_event_properties!(CharWidthChangedEvent);
impl_from_object_ref!(CharWidthChangedEvent);

impl_msg_conversion_ext_for_target_type!(LineChangedEvent);
impl_msg_conversion_ext_for_target_type!(ColumnCountChangedEvent);
impl_msg_conversion_ext_for_target_type!(LineCountChangedEvent);
impl_msg_conversion_ext_for_target_type!(ApplicationChangedEvent);
impl_msg_conversion_ext_for_target_type!(CharWidthChangedEvent);

impl_msg_conversion_for_types_built_from_object_ref!(LineChangedEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ColumnCountChangedEvent);
impl_msg_conversion_for_types_built_from_object_ref!(LineCountChangedEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ApplicationChangedEvent);
impl_msg_conversion_for_types_built_from_object_ref!(CharWidthChangedEvent);
