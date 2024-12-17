#[cfg(feature = "zbus")]
use crate::events::{
	EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
};
use crate::{
	error::AtspiError,
	events::{BusProperties, HasInterfaceName, HasMatchRule, HasRegistryEventString, document::*},
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

impl HasInterfaceName for DocumentEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for DocumentEvents {
	fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError> {
		let header = msg.header();
		let member = header.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			LoadCompleteEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::LoadComplete(LoadCompleteEvent::from_message_unchecked(msg)?))
			}
			ReloadEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::Reload(ReloadEvent::from_message_unchecked(msg)?))
			}
			LoadStoppedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::LoadStopped(LoadStoppedEvent::from_message_unchecked(msg)?))
			}
			ContentChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::ContentChanged(
				ContentChangedEvent::from_message_unchecked(msg)?,
			)),
			AttributesChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::AttributesChanged(
				AttributesChangedEvent::from_message_unchecked(msg)?,
			)),
			PageChangedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::PageChanged(PageChangedEvent::from_message_unchecked(msg)?))
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
impl_try_from_event_for_user_facing_type!(
	LoadCompleteEvent,
	DocumentEvents::LoadComplete,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	ReloadEvent,
	DocumentEvents,
	DocumentEvents::Reload
);
impl_try_from_event_for_user_facing_type!(ReloadEvent, DocumentEvents::Reload, Event::Document);

impl_from_user_facing_event_for_interface_event_enum!(
	LoadStoppedEvent,
	DocumentEvents,
	DocumentEvents::LoadStopped
);
impl_try_from_event_for_user_facing_type!(
	LoadStoppedEvent,
	DocumentEvents::LoadStopped,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	ContentChangedEvent,
	DocumentEvents,
	DocumentEvents::ContentChanged
);
impl_try_from_event_for_user_facing_type!(
	ContentChangedEvent,
	DocumentEvents::ContentChanged,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	AttributesChangedEvent,
	DocumentEvents,
	DocumentEvents::AttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	AttributesChangedEvent,
	DocumentEvents::AttributesChanged,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	PageChangedEvent,
	DocumentEvents,
	DocumentEvents::PageChanged
);
impl_try_from_event_for_user_facing_type!(
	PageChangedEvent,
	DocumentEvents::PageChanged,
	Event::Document
);

impl HasRegistryEventString for DocumentEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}
