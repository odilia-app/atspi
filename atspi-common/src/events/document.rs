#[cfg(feature = "zbus")]
use crate::events::{EventWrapperMessageConversion, MessageConversion, TryFromMessage, MessageConversionExt};
use crate::{
	error::AtspiError,
	events::{BusProperties, HasInterfaceName, HasMatchRule, HasRegistryEventString},
	Event, EventProperties, EventTypeProperties,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum DocumentEvents {
	/// See: [`LoadCompleteEvent`].
	LoadComplete(LoadCompleteEvent),
	/// See: [`ReloadEvent`].
	Reload(ReloadEvent),
	/// See: [`LoadStoppedEvent`].
	LoadStopped(LoadStoppedEvent),
	/// See: [`ContentChangedEvent`].
	ContentChanged(ContentChangedEvent),
	/// See: [`AttributesChangedEvent`].
	AttributesChanged(AttributesChangedEvent),
	/// See: [`PageChangedEvent`].
	PageChanged(PageChangedEvent),
}

impl EventTypeProperties for DocumentEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.member(),
			Self::Reload(inner) => inner.member(),
			Self::LoadStopped(inner) => inner.member(),
			Self::ContentChanged(inner) => inner.member(),
			Self::AttributesChanged(inner) => inner.member(),
			Self::PageChanged(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.interface(),
			Self::Reload(inner) => inner.interface(),
			Self::LoadStopped(inner) => inner.interface(),
			Self::ContentChanged(inner) => inner.interface(),
			Self::AttributesChanged(inner) => inner.interface(),
			Self::PageChanged(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.match_rule(),
			Self::Reload(inner) => inner.match_rule(),
			Self::LoadStopped(inner) => inner.match_rule(),
			Self::ContentChanged(inner) => inner.match_rule(),
			Self::AttributesChanged(inner) => inner.match_rule(),
			Self::PageChanged(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.registry_string(),
			Self::Reload(inner) => inner.registry_string(),
			Self::LoadStopped(inner) => inner.registry_string(),
			Self::ContentChanged(inner) => inner.registry_string(),
			Self::AttributesChanged(inner) => inner.registry_string(),
			Self::PageChanged(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for DocumentEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::LoadComplete(inner) => inner.path(),
			Self::Reload(inner) => inner.path(),
			Self::LoadStopped(inner) => inner.path(),
			Self::ContentChanged(inner) => inner.path(),
			Self::AttributesChanged(inner) => inner.path(),
			Self::PageChanged(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::LoadComplete(inner) => inner.sender(),
			Self::Reload(inner) => inner.sender(),
			Self::LoadStopped(inner) => inner.sender(),
			Self::ContentChanged(inner) => inner.sender(),
			Self::AttributesChanged(inner) => inner.sender(),
			Self::PageChanged(inner) => inner.sender(),
		}
	}
}

impl_from_interface_event_enum_for_event!(DocumentEvents, Event::Document);
impl_try_from_event_for_user_facing_event_type!(DocumentEvents, Event::Document);
event_wrapper_test_cases!(DocumentEvents, LoadCompleteEvent);

impl HasMatchRule for DocumentEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document'";
}

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

/// The focused page has changed. This event is usually sent only by document readers, signaling
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

impl HasInterfaceName for DocumentEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for DocumentEvents {
	fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let header = msg.header();
		let member = header.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			LoadCompleteEvent::DBUS_MEMBER => Ok(DocumentEvents::LoadComplete(
				LoadCompleteEvent::try_from_validated_message(msg)?,
			)),
			ReloadEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::Reload(ReloadEvent::try_from_validated_message(msg)?))
			}
			LoadStoppedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::LoadStopped(LoadStoppedEvent::try_from_validated_message(msg)?))
			}
			ContentChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::ContentChanged(
				ContentChangedEvent::try_from_validated_message(msg)?,
			)),
			AttributesChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::AttributesChanged(
				AttributesChangedEvent::try_from_validated_message(msg)?,
			)),
			PageChangedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::PageChanged(PageChangedEvent::try_from_validated_message(msg)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for DocumentEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LoadCompleteEvent,
	DocumentEvents,
	DocumentEvents::LoadComplete
);
impl_from_user_facing_type_for_event_enum!(LoadCompleteEvent, Event::Document);
impl_try_from_event_for_user_facing_type!(
	LoadCompleteEvent,
	DocumentEvents::LoadComplete,
	Event::Document
);

event_test_cases!(LoadCompleteEvent);
impl_to_dbus_message!(LoadCompleteEvent);
impl_from_dbus_message!(LoadCompleteEvent);
impl_event_properties!(LoadCompleteEvent);
impl_from_object_ref!(LoadCompleteEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	ReloadEvent,
	DocumentEvents,
	DocumentEvents::Reload
);
impl_from_user_facing_type_for_event_enum!(ReloadEvent, Event::Document);
impl_try_from_event_for_user_facing_type!(ReloadEvent, DocumentEvents::Reload, Event::Document);

event_test_cases!(ReloadEvent);
impl_to_dbus_message!(ReloadEvent);
impl_from_dbus_message!(ReloadEvent);
impl_event_properties!(ReloadEvent);
impl_from_object_ref!(ReloadEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	LoadStoppedEvent,
	DocumentEvents,
	DocumentEvents::LoadStopped
);
impl_from_user_facing_type_for_event_enum!(LoadStoppedEvent, Event::Document);
impl_try_from_event_for_user_facing_type!(
	LoadStoppedEvent,
	DocumentEvents::LoadStopped,
	Event::Document
);

event_test_cases!(LoadStoppedEvent);
impl_to_dbus_message!(LoadStoppedEvent);
impl_from_dbus_message!(LoadStoppedEvent);
impl_event_properties!(LoadStoppedEvent);
impl_from_object_ref!(LoadStoppedEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	ContentChangedEvent,
	DocumentEvents,
	DocumentEvents::ContentChanged
);
impl_from_user_facing_type_for_event_enum!(ContentChangedEvent, Event::Document);
impl_try_from_event_for_user_facing_type!(
	ContentChangedEvent,
	DocumentEvents::ContentChanged,
	Event::Document
);

event_test_cases!(ContentChangedEvent);
impl_to_dbus_message!(ContentChangedEvent);
impl_from_dbus_message!(ContentChangedEvent);
impl_event_properties!(ContentChangedEvent);
impl_from_object_ref!(ContentChangedEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	AttributesChangedEvent,
	DocumentEvents,
	DocumentEvents::AttributesChanged
);
impl_from_user_facing_type_for_event_enum!(AttributesChangedEvent, Event::Document);
impl_try_from_event_for_user_facing_type!(
	AttributesChangedEvent,
	DocumentEvents::AttributesChanged,
	Event::Document
);

event_test_cases!(AttributesChangedEvent);
impl_to_dbus_message!(AttributesChangedEvent);
impl_from_dbus_message!(AttributesChangedEvent);
impl_event_properties!(AttributesChangedEvent);
impl_from_object_ref!(AttributesChangedEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	PageChangedEvent,
	DocumentEvents,
	DocumentEvents::PageChanged
);
impl_from_user_facing_type_for_event_enum!(PageChangedEvent, Event::Document);
impl_try_from_event_for_user_facing_type!(
	PageChangedEvent,
	DocumentEvents::PageChanged,
	Event::Document
);

event_test_cases!(PageChangedEvent);
impl_to_dbus_message!(PageChangedEvent);
impl_from_dbus_message!(PageChangedEvent);
impl_event_properties!(PageChangedEvent);
impl_from_object_ref!(PageChangedEvent);

impl HasRegistryEventString for DocumentEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}
