#[cfg(feature = "zbus")]
use crate::events::{
	EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
};
use crate::{
	error::AtspiError,
	events::{BusProperties, HasInterfaceName, HasMatchRule, HasRegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
pub use crate::events::wrappers::DocumentEvents;
use zbus_names::UniqueName;
use zvariant::ObjectPath;

/// An event triggered by the completion of a document load action.
/// For example: a web page has finished loading its initial payload, or
/// `LibreOffice` has loaded a document from disk.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LoadCompleteEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// An event triggered by a reloading of a document.
/// For example: pressing F5, or `Control + r` will reload a page in a web browser.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReloadEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// An event triggered by the cancelling of a document load.
/// For example: during the loading of a large web page, a user may press `Escape` to stop loading the page.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LoadStoppedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ContentChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AttributesChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

/// The focused page has changed.
///
/// This event is usually sent only by document readers, signaling
/// that the _physical page equivalent is now different.
/// This event does not encode _which_ page is the new one, only that a new page is now the primary
/// one.
///
/// See `atspi_proxies::document::DocumentProxy::current_page_number` to actively find the
/// page number.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PageChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: crate::events::ObjectRef,
}

impl BusProperties for LoadCompleteEvent {
	const DBUS_MEMBER: &'static str = "LoadComplete";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl BusProperties for ReloadEvent {
	const DBUS_MEMBER: &'static str = "Reload";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='Reload'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl BusProperties for LoadStoppedEvent {
	const DBUS_MEMBER: &'static str = "LoadStopped";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl BusProperties for ContentChangedEvent {
	const DBUS_MEMBER: &'static str = "ContentChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl BusProperties for AttributesChangedEvent {
	const DBUS_MEMBER: &'static str = "AttributesChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl BusProperties for PageChangedEvent {
	const DBUS_MEMBER: &'static str = "PageChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl_from_user_facing_type_for_event_enum!(LoadCompleteEvent, Event::Document);

event_test_cases!(LoadCompleteEvent);
impl_to_dbus_message!(LoadCompleteEvent);
impl_from_dbus_message!(LoadCompleteEvent);
impl_event_properties!(LoadCompleteEvent);
impl_from_object_ref!(LoadCompleteEvent);

impl_from_user_facing_type_for_event_enum!(ReloadEvent, Event::Document);

event_test_cases!(ReloadEvent);
impl_to_dbus_message!(ReloadEvent);
impl_from_dbus_message!(ReloadEvent);
impl_event_properties!(ReloadEvent);
impl_from_object_ref!(ReloadEvent);

impl_from_user_facing_type_for_event_enum!(LoadStoppedEvent, Event::Document);

event_test_cases!(LoadStoppedEvent);
impl_to_dbus_message!(LoadStoppedEvent);
impl_from_dbus_message!(LoadStoppedEvent);
impl_event_properties!(LoadStoppedEvent);
impl_from_object_ref!(LoadStoppedEvent);

impl_from_user_facing_type_for_event_enum!(ContentChangedEvent, Event::Document);

event_test_cases!(ContentChangedEvent);
impl_to_dbus_message!(ContentChangedEvent);
impl_from_dbus_message!(ContentChangedEvent);
impl_event_properties!(ContentChangedEvent);
impl_from_object_ref!(ContentChangedEvent);

impl_from_user_facing_type_for_event_enum!(AttributesChangedEvent, Event::Document);

event_test_cases!(AttributesChangedEvent);
impl_to_dbus_message!(AttributesChangedEvent);
impl_from_dbus_message!(AttributesChangedEvent);
impl_event_properties!(AttributesChangedEvent);
impl_from_object_ref!(AttributesChangedEvent);

impl_from_user_facing_type_for_event_enum!(PageChangedEvent, Event::Document);

event_test_cases!(PageChangedEvent);
impl_to_dbus_message!(PageChangedEvent);
impl_from_dbus_message!(PageChangedEvent);
impl_event_properties!(PageChangedEvent);
impl_from_object_ref!(PageChangedEvent);

