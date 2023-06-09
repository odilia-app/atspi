use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that this example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi::events::Event;
/// use atspi::events::document::LoadCompleteEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<LoadCompleteEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let output = std::process::Command::new("busctl")
/// #       .arg("--user")
/// #       .arg("call")
/// #       .arg("org.a11y.Bus")
/// #       .arg("/org/a11y/bus")
/// #       .arg("org.a11y.Bus")
/// #       .arg("GetAddress")
/// #       .output()
/// #       .unwrap();
/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
/// #    let addr_str = addr_string
/// #        .strip_prefix("s \"")
/// #        .unwrap()
/// #        .trim()
/// #        .strip_suffix('"')
/// #        .unwrap();
/// #   let mut base_cmd = std::process::Command::new("busctl");
/// #   let thing = base_cmd
/// #       .arg("--address")
/// #       .arg(addr_str)
/// #       .arg("emit")
/// #       .arg("/org/a11y/atspi/accessible/null")
/// #       .arg("org.a11y.atspi.Event.Document")
/// #       .arg("LoadComplete")
/// #       .arg("siiva{sv}")
/// #       .arg("")
/// #       .arg("0")
/// #       .arg("0")
/// #       .arg("i")
/// #       .arg("0")
/// #       .arg("0")
/// #       .output()
/// #       .unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///          if let Event::Document(_event) = ev {
/// #            break;
///              // do things with your event here
///          }
/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum DocumentEvents {
	LoadComplete(LoadCompleteEvent),
	Reload(ReloadEvent),
	LoadStopped(LoadStoppedEvent),
	ContentChanged(ContentChangedEvent),
	AttributesChanged(AttributesChangedEvent),
	PageChanged(PageChangedEvent),
}
impl_event_conversions!(DocumentEvents, Event::Document);
event_wrapper_test_cases!(DocumentEvents, LoadCompleteEvent);

impl HasMatchRule for DocumentEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document'";
}

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that the example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi::events::Event;
/// # use atspi::events::GenericEvent;
/// use atspi::events::document::LoadCompleteEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<LoadCompleteEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = LoadCompleteEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = LoadCompleteEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LoadCompleteEvent {
	pub item: crate::events::Accessible,
}

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that the example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi::events::Event;
/// # use atspi::events::GenericEvent;
/// use atspi::events::document::ReloadEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<ReloadEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = ReloadEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = ReloadEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReloadEvent {
	pub item: crate::events::Accessible,
}

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that the example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi::events::Event;
/// # use atspi::events::GenericEvent;
/// use atspi::events::document::LoadStoppedEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<LoadStoppedEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = LoadStoppedEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = LoadStoppedEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LoadStoppedEvent {
	pub item: crate::events::Accessible,
}

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that the example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi::events::Event;
/// # use atspi::events::GenericEvent;
/// use atspi::events::document::ContentChangedEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<ContentChangedEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = ContentChangedEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = ContentChangedEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ContentChangedEvent {
	pub item: crate::events::Accessible,
}

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that the example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi::events::Event;
/// # use atspi::events::GenericEvent;
/// use atspi::events::document::AttributesChangedEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<AttributesChangedEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = AttributesChangedEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = AttributesChangedEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AttributesChangedEvent {
	pub item: crate::events::Accessible,
}

// IgnoreBlock start
/// # Example
///
/// Even though this example employs `Tokio`, any runtime will do.
///
/// Note that the example is minimized for rhe sake of brevity.
/// More complete examples may be found in the `examples/` directory.
///
/// ```
/// use atspi::events::Event;
/// # use atspi::events::GenericEvent;
/// use atspi::events::document::PageChangedEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<PageChangedEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = PageChangedEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = PageChangedEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PageChangedEvent {
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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

/*
impl TryFrom<Event> for LoadCompleteEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Document(DocumentEvents::LoadComplete(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

/*
impl TryFrom<Event> for ReloadEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Document(DocumentEvents::Reload(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

/*
impl TryFrom<Event> for LoadStoppedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Document(DocumentEvents::LoadStopped(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

/*
impl TryFrom<Event> for ContentChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Document(DocumentEvents::ContentChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

/*
impl TryFrom<Event> for AttributesChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Document(DocumentEvents::AttributesChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

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
	fn sender(&self) -> UniqueName<'_> {
		self.item.name.clone().into()
	}
	fn path<'a>(&self) -> ObjectPath<'_> {
		self.item.path.clone().into()
	}
	fn body(&self) -> Self::Body {
		let copy = self.clone();
		copy.into()
	}
}

/*
impl TryFrom<Event> for PageChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Document(DocumentEvents::PageChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

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

impl_event_conversions!(
	LoadCompleteEvent,
	DocumentEvents,
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

impl_event_conversions!(ReloadEvent, DocumentEvents, DocumentEvents::Reload, Event::Document);
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

impl_event_conversions!(
	LoadStoppedEvent,
	DocumentEvents,
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

impl_event_conversions!(
	ContentChangedEvent,
	DocumentEvents,
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

impl_event_conversions!(
	AttributesChangedEvent,
	DocumentEvents,
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

impl_event_conversions!(
	PageChangedEvent,
	DocumentEvents,
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

/*impl HasMatchRule for LoadCompleteEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'";
}*/
/*impl HasMatchRule for ReloadEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='Reload'";
}*/
/*impl HasMatchRule for LoadStoppedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'";
}*/
/*impl HasMatchRule for ContentChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'";
}*/
/*impl HasMatchRule for AttributesChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'";
}*/
/*impl HasMatchRule for PageChangedEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'";
}*/
/*impl HasRegistryEventString for LoadCompleteEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Document:LoadComplete";
}*/
/*impl HasRegistryEventString for ReloadEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Document:Reload";
}*/
/*impl HasRegistryEventString for LoadStoppedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Document:LoadStopped";
}*/
/*impl HasRegistryEventString for ContentChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Document:ContentChanged";
}*/
/*impl HasRegistryEventString for AttributesChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Document:AttributesChanged";
}*/
/*impl HasRegistryEventString for PageChangedEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Document:PageChanged";
}*/
impl HasRegistryEventString for DocumentEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}
