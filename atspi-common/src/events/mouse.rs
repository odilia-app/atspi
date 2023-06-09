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
/// use atspi::events::mouse::AbsEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<AbsEvent>().await.unwrap();
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
/// #       .arg("org.a11y.atspi.Event.Mouse")
/// #       .arg("Abs")
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
///          if let Event::Mouse(_event) = ev {
/// #            break;
///              // do things with your event here
///          }
/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MouseEvents {
	Abs(AbsEvent),
	Rel(RelEvent),
	Button(ButtonEvent),
}
impl_event_conversions!(MouseEvents, Event::Mouse);
event_wrapper_test_cases!(MouseEvents, AbsEvent);

impl HasMatchRule for MouseEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse'";
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
/// use atspi::events::mouse::AbsEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<AbsEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = AbsEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = AbsEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AbsEvent {
	pub item: crate::events::Accessible,
	pub x: i32,
	pub y: i32,
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
/// use atspi::events::mouse::RelEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<RelEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = RelEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = RelEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RelEvent {
	pub item: crate::events::Accessible,
	pub x: i32,
	pub y: i32,
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
/// use atspi::events::mouse::ButtonEvent;
/// # use std::time::Duration;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<ButtonEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = ButtonEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = ButtonEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ButtonEvent {
	pub item: crate::events::Accessible,
	pub detail: String,
	pub mouse_x: i32,
	pub mouse_y: i32,
}

impl GenericEvent<'_> for AbsEvent {
	const DBUS_MEMBER: &'static str = "Abs";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, x: body.detail1, y: body.detail2 })
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
impl TryFrom<Event> for AbsEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Mouse(MouseEvents::Abs(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for RelEvent {
	const DBUS_MEMBER: &'static str = "Rel";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, x: body.detail1, y: body.detail2 })
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
impl TryFrom<Event> for RelEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Mouse(MouseEvents::Rel(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ButtonEvent {
	const DBUS_MEMBER: &'static str = "Button";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, detail: body.kind, mouse_x: body.detail1, mouse_y: body.detail2 })
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
impl TryFrom<Event> for ButtonEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Mouse(MouseEvents::Button(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for MouseEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"Abs" => Ok(MouseEvents::Abs(ev.try_into()?)),
			"Rel" => Ok(MouseEvents::Rel(ev.try_into()?)),
			"Button" => Ok(MouseEvents::Button(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
		}
	}
}

impl_event_conversions!(AbsEvent, MouseEvents, MouseEvents::Abs, Event::Mouse);
event_test_cases!(AbsEvent);
impl_to_dbus_message!(AbsEvent);
impl_from_dbus_message!(AbsEvent);
impl From<AbsEvent> for EventBodyOwned {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.x,
			detail2: event.y,
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(RelEvent, MouseEvents, MouseEvents::Rel, Event::Mouse);
event_test_cases!(RelEvent);
impl_to_dbus_message!(RelEvent);
impl_from_dbus_message!(RelEvent);
impl From<RelEvent> for EventBodyOwned {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.x,
			detail2: event.y,
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(ButtonEvent, MouseEvents, MouseEvents::Button, Event::Mouse);
event_test_cases!(ButtonEvent);
impl_to_dbus_message!(ButtonEvent);
impl_from_dbus_message!(ButtonEvent);
impl From<ButtonEvent> for EventBodyOwned {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.detail,
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

/*impl HasMatchRule for AbsEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
}*/
/*impl HasMatchRule for RelEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
}*/
/*impl HasMatchRule for ButtonEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
}*/
/*impl HasRegistryEventString for AbsEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:Abs";
}*/
/*impl HasRegistryEventString for RelEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:Rel";
}*/
/*impl HasRegistryEventString for ButtonEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:Button";
}*/
impl HasRegistryEventString for MouseEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
}
