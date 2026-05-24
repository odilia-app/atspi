use crate::events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString};
#[cfg(feature = "zbus")]
use crate::EventProperties;
use crate::NonNullObjectRef;

#[cfg(feature = "zbus")]
use crate::AtspiError;

/// An event triggered by the completion of a document load action.
/// For example: a web page has finished loading its initial payload, or
/// `LibreOffice` has loaded a document from disk.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct LoadCompleteEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

/// An event triggered by a reloading of a document.
/// For example: pressing F5, or `Control + r` will reload a page in a web browser.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ReloadEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

/// An event triggered by the cancelling of a document load.
/// For example: during the loading of a large web page, a user may press `Escape` to stop loading the page.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct LoadStoppedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ContentChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct AttributesChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

/// The focused page has changed.
///
/// This event is usually sent only by document readers, signaling
/// that the physical page equivalent is now different.
/// This event does not encode _which_ page is the new one, only that a new page is now the primary
/// one.
///
/// See `atspi_proxies::document::DocumentProxy::current_page_number` to actively find the
/// page number.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct PageChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_test_event!(
	LoadCompleteEvent<'_>,
	ReloadEvent<'_>,
	LoadStoppedEvent<'_>,
	PageChangedEvent<'_>,
	ContentChangedEvent<'_>,
	AttributesChangedEvent<'_>
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LoadCompleteEvent<'_>,
	"LoadComplete",
	"org.a11y.atspi.Event.Document",
	"document:load-complete",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ReloadEvent<'_>,
	"Reload",
	"org.a11y.atspi.Event.Document",
	"document:reload",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LoadStoppedEvent<'_>,
	"LoadStopped",
	"org.a11y.atspi.Event.Document",
	"document:load-stopped",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'"
);

// TODO confirm registry event string, not found in grep at at-spi2-core
impl_member_interface_registry_string_and_match_rule_for_event!(
	ContentChangedEvent<'_>,
	"ContentChanged",
	"org.a11y.atspi.Event.Document",
	"document:content-changed",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	AttributesChangedEvent<'_>,
	"AttributesChanged",
	"org.a11y.atspi.Event.Document",
	"document:attributes-changed",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	PageChangedEvent<'_>,
	"PageChanged",
	"org.a11y.atspi.Event.Document",
	"document:page-changed",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'"
);

impl_event_type_properties_for_event!(LoadCompleteEvent<'_>);
event_test_cases!(LoadCompleteEvent);
impl_to_dbus_message!(LoadCompleteEvent<'_>);
impl_event_properties!(LoadCompleteEvent<'_>);
impl_from_dbus_message!(LoadCompleteEvent<'_>);
impl_from_object_ref!(LoadCompleteEvent<'_>);

impl_event_type_properties_for_event!(ReloadEvent<'_>);
event_test_cases!(ReloadEvent);
impl_to_dbus_message!(ReloadEvent<'_>);
impl_from_dbus_message!(ReloadEvent<'_>);
impl_event_properties!(ReloadEvent<'_>);
impl_from_object_ref!(ReloadEvent<'_>);

impl_event_type_properties_for_event!(LoadStoppedEvent<'_>);
event_test_cases!(LoadStoppedEvent);
impl_to_dbus_message!(LoadStoppedEvent<'_>);
impl_from_dbus_message!(LoadStoppedEvent<'_>);
impl_event_properties!(LoadStoppedEvent<'_>);
impl_from_object_ref!(LoadStoppedEvent<'_>);

impl_event_type_properties_for_event!(ContentChangedEvent<'_>);
event_test_cases!(ContentChangedEvent);
impl_to_dbus_message!(ContentChangedEvent<'_>);
impl_from_dbus_message!(ContentChangedEvent<'_>);
impl_event_properties!(ContentChangedEvent<'_>);
impl_from_object_ref!(ContentChangedEvent<'_>);

impl_event_type_properties_for_event!(AttributesChangedEvent<'_>);
event_test_cases!(AttributesChangedEvent);
impl_to_dbus_message!(AttributesChangedEvent<'_>);
impl_from_dbus_message!(AttributesChangedEvent<'_>);
impl_event_properties!(AttributesChangedEvent<'_>);
impl_from_object_ref!(AttributesChangedEvent<'_>);

impl_event_type_properties_for_event!(PageChangedEvent<'_>);
event_test_cases!(PageChangedEvent);
impl_to_dbus_message!(PageChangedEvent<'_>);
impl_from_dbus_message!(PageChangedEvent<'_>);
impl_event_properties!(PageChangedEvent<'_>);
impl_from_object_ref!(PageChangedEvent<'_>);

impl_msg_conversion_ext_for_target_type!(
	LoadCompleteEvent<'_>,
	ReloadEvent<'_>,
	LoadStoppedEvent<'_>,
	ContentChangedEvent<'_>,
	AttributesChangedEvent<'_>,
	PageChangedEvent<'_>,
);

impl_msg_conversion_for_types_built_from_object_ref!(
	LoadCompleteEvent<'_>,
	ReloadEvent<'_>,
	LoadStoppedEvent<'_>,
	ContentChangedEvent<'_>,
	AttributesChangedEvent<'_>,
	PageChangedEvent<'_>,
);
