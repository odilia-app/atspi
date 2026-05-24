#[cfg(feature = "zbus")]
use crate::error::AtspiError;
#[cfg(feature = "zbus")]
use crate::EventProperties;
use crate::{
	events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString},
	object_ref::NonNullObjectRef,
};

/// A line of text has been changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct LineChangedEvent<'a> {
	/// The [`crate::ObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(LineChangedEvent<'_>);

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ColumnCountChangedEvent<'a> {
	/// The [`crate::ObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ColumnCountChangedEvent<'_>);

/// The height of a terminal emulator has changed sufficiently such that the number of lines
/// able to fit within the terminal has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct LineCountChangedEvent<'a> {
	/// The [`crate::ObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(LineCountChangedEvent<'_>);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ApplicationChangedEvent<'a> {
	/// The [`crate::ObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ApplicationChangedEvent<'_>);

/// The width of a terminal emulator has changed sufficiently such that the number of characters
/// able to fit on one *visual* line has changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct CharWidthChangedEvent<'a> {
	/// The [`crate::ObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(CharWidthChangedEvent<'_>);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LineChangedEvent<'_>,
	"LineChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:line-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ColumnCountChangedEvent<'_>,
	"ColumncountChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:columncount-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LineCountChangedEvent<'_>,
	"LinecountChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:linecount-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ApplicationChangedEvent<'_>,
	"ApplicationChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:application-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	CharWidthChangedEvent<'_>,
	"CharwidthChanged",
	"org.a11y.atspi.Event.Terminal",
	"terminal:char-width-changed",
	"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'"
);

event_test_cases!(LineChangedEvent);
impl_to_dbus_message!(LineChangedEvent<'_>);
impl_from_dbus_message!(LineChangedEvent<'_>);
impl_event_properties!(LineChangedEvent<'_>);
impl_from_object_ref!(LineChangedEvent<'_>);

event_test_cases!(ColumnCountChangedEvent);
impl_to_dbus_message!(ColumnCountChangedEvent<'_>);
impl_from_dbus_message!(ColumnCountChangedEvent<'_>);
impl_event_properties!(ColumnCountChangedEvent<'_>);
impl_from_object_ref!(ColumnCountChangedEvent<'_>);

event_test_cases!(LineCountChangedEvent);
impl_to_dbus_message!(LineCountChangedEvent<'_>);
impl_from_dbus_message!(LineCountChangedEvent<'_>);
impl_event_properties!(LineCountChangedEvent<'_>);
impl_from_object_ref!(LineCountChangedEvent<'_>);

event_test_cases!(ApplicationChangedEvent);
impl_to_dbus_message!(ApplicationChangedEvent<'_>);
impl_from_dbus_message!(ApplicationChangedEvent<'_>);
impl_event_properties!(ApplicationChangedEvent<'_>);
impl_from_object_ref!(ApplicationChangedEvent<'_>);

event_test_cases!(CharWidthChangedEvent);
impl_to_dbus_message!(CharWidthChangedEvent<'_>);
impl_from_dbus_message!(CharWidthChangedEvent<'_>);
impl_event_properties!(CharWidthChangedEvent<'_>);
impl_from_object_ref!(CharWidthChangedEvent<'_>);

impl_msg_conversion_ext_for_target_type!(
	LineChangedEvent<'_>,
	ColumnCountChangedEvent<'_>,
	LineCountChangedEvent<'_>,
	ApplicationChangedEvent<'_>,
	CharWidthChangedEvent<'_>,
);

impl_msg_conversion_for_types_built_from_object_ref!(
	LineChangedEvent<'_>,
	ColumnCountChangedEvent<'_>,
	LineCountChangedEvent<'_>,
	ApplicationChangedEvent<'_>,
	CharWidthChangedEvent<'_>,
);

impl_test_event!(
	LineChangedEvent<'_>,
	ColumnCountChangedEvent<'_>,
	LineCountChangedEvent<'_>,
	ApplicationChangedEvent<'_>,
	CharWidthChangedEvent<'_>,
);
