use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
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
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// An event triggered by a reloading of a document.
/// For example: pressing F5, or `Control + r` will reload a page in a web browser.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReloadEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// An event triggered by the cancelling of a document load.
/// For example: during the loading of a large web page, a user may press `Escape` to stop loading the page.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LoadStoppedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ContentChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AttributesChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PageChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

impl GenericEvent<'_> for LoadCompleteEvent {
	const DBUS_MEMBER: &'static str = "LoadComplete";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for ReloadEvent {
	const DBUS_MEMBER: &'static str = "Reload";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='Reload'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for LoadStoppedEvent {
	const DBUS_MEMBER: &'static str = "LoadStopped";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for ContentChangedEvent {
	const DBUS_MEMBER: &'static str = "ContentChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for AttributesChangedEvent {
	const DBUS_MEMBER: &'static str = "AttributesChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

impl GenericEvent<'_> for PageChangedEvent {
	const DBUS_MEMBER: &'static str = "PageChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Document:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item })
	}
	fn sender(&self) -> String {
		self.item.name.clone()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for DocumentEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"LoadComplete" => Ok(DocumentEvents::LoadComplete(ev.try_into()?)),
			"Reload" => Ok(DocumentEvents::Reload(ev.try_into()?)),
			"LoadStopped" => Ok(DocumentEvents::LoadStopped(ev.try_into()?)),
			"ContentChanged" => Ok(DocumentEvents::ContentChanged(ev.try_into()?)),
			"AttributesChanged" => Ok(DocumentEvents::AttributesChanged(ev.try_into()?)),
			"PageChanged" => Ok(DocumentEvents::PageChanged(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
		}
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
impl From<LoadCompleteEvent> for EventBodyOwned {
	fn from(_event: LoadCompleteEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

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
impl From<ReloadEvent> for EventBodyOwned {
	fn from(_event: ReloadEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

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
impl From<LoadStoppedEvent> for EventBodyOwned {
	fn from(_event: LoadStoppedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

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
impl From<ContentChangedEvent> for EventBodyOwned {
	fn from(_event: ContentChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

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
impl From<AttributesChangedEvent> for EventBodyOwned {
	fn from(_event: AttributesChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

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
impl From<PageChangedEvent> for EventBodyOwned {
	fn from(_event: PageChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl HasRegistryEventString for DocumentEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}
