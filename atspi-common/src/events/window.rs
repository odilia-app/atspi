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
/// use atspi_common::events::Event;
/// use atspi_common::events::window::PropertyChangeEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<PropertyChangeEvent>().await.unwrap();
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
/// #       .arg("org.a11y.atspi.Event.Window")
/// #       .arg("PropertyChange")
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
///          if let Event::Window(_event) = ev {
/// #            break;
///              // do things with your event here
///          }
/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum WindowEvents {
	PropertyChange(PropertyChangeEvent),
	Minimize(MinimizeEvent),
	Maximize(MaximizeEvent),
	Restore(RestoreEvent),
	Close(CloseEvent),
	Create(CreateEvent),
	Reparent(ReparentEvent),
	DesktopCreate(DesktopCreateEvent),
	DesktopDestroy(DesktopDestroyEvent),
	Destroy(DestroyEvent),
	Activate(ActivateEvent),
	Deactivate(DeactivateEvent),
	Raise(RaiseEvent),
	Lower(LowerEvent),
	Move(MoveEvent),
	Resize(ResizeEvent),
	Shade(ShadeEvent),
	UUshade(UUshadeEvent),
	Restyle(RestyleEvent),
}
impl_event_conversions!(WindowEvents, Event::Window);
event_wrapper_test_cases!(WindowEvents, MoveEvent);

impl HasMatchRule for WindowEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window'";
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::PropertyChangeEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<PropertyChangeEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = PropertyChangeEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = PropertyChangeEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PropertyChangeEvent {
	pub item: crate::events::Accessible,
	pub property: String,
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::MinimizeEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<MinimizeEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = MinimizeEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = MinimizeEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MinimizeEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::MaximizeEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<MaximizeEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = MaximizeEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = MaximizeEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MaximizeEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::RestoreEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<RestoreEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = RestoreEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = RestoreEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RestoreEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::CloseEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<CloseEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = CloseEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = CloseEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CloseEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::CreateEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<CreateEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = CreateEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = CreateEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CreateEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::ReparentEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<ReparentEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = ReparentEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = ReparentEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReparentEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::DesktopCreateEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<DesktopCreateEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = DesktopCreateEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = DesktopCreateEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DesktopCreateEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::DesktopDestroyEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<DesktopDestroyEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = DesktopDestroyEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = DesktopDestroyEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DesktopDestroyEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::DestroyEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<DestroyEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = DestroyEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = DestroyEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DestroyEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::ActivateEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<ActivateEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = ActivateEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = ActivateEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ActivateEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::DeactivateEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<DeactivateEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = DeactivateEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = DeactivateEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DeactivateEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::RaiseEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<RaiseEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = RaiseEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = RaiseEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RaiseEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::LowerEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<LowerEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = LowerEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = LowerEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LowerEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::MoveEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<MoveEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = MoveEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = MoveEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MoveEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::ResizeEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<ResizeEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = ResizeEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = ResizeEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ResizeEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::ShadeEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<ShadeEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = ShadeEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = ShadeEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ShadeEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::UUshadeEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<UUshadeEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = UUshadeEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = UUshadeEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct UUshadeEvent {
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
/// use atspi_common::events::Event;
/// # use atspi_common::events::GenericEvent;
/// use atspi_common::events::window::RestyleEvent;
/// # use std::time::Duration;
/// use futures_lite::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();
///     let mut events = atspi.event_stream();
/// #   atspi.register_event::<RestyleEvent>().await.unwrap();
///     std::pin::pin!(&mut events);
/// #   let event_struct = RestyleEvent::default();
/// #   atspi.send_event(event_struct.clone()).await.unwrap();
///
///     while let Some(Ok(ev)) = events.next().await {
///         if let Ok(event) = RestyleEvent::try_from(ev) {
/// #          assert_eq!(event.body(), event_struct.body());
/// #          break;
///            // do something with the specific event you've received
///         } else { continue };
///     }
/// }
/// ```
// IgnoreBlock stop
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RestyleEvent {
	pub item: crate::events::Accessible,
}

impl GenericEvent<'_> for PropertyChangeEvent {
	const DBUS_MEMBER: &'static str = "PropertyChange";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, property: body.kind })
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
impl TryFrom<Event> for PropertyChangeEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::PropertyChange(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for MinimizeEvent {
	const DBUS_MEMBER: &'static str = "Minimize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for MinimizeEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Minimize(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for MaximizeEvent {
	const DBUS_MEMBER: &'static str = "Maximize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for MaximizeEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Maximize(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for RestoreEvent {
	const DBUS_MEMBER: &'static str = "Restore";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for RestoreEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Restore(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for CloseEvent {
	const DBUS_MEMBER: &'static str = "Close";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for CloseEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Close(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for CreateEvent {
	const DBUS_MEMBER: &'static str = "Create";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for CreateEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Create(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ReparentEvent {
	const DBUS_MEMBER: &'static str = "Reparent";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for ReparentEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Reparent(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for DesktopCreateEvent {
	const DBUS_MEMBER: &'static str = "DesktopCreate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for DesktopCreateEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::DesktopCreate(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for DesktopDestroyEvent {
	const DBUS_MEMBER: &'static str = "DesktopDestroy";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for DesktopDestroyEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::DesktopDestroy(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for DestroyEvent {
	const DBUS_MEMBER: &'static str = "Destroy";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for DestroyEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Destroy(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ActivateEvent {
	const DBUS_MEMBER: &'static str = "Activate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for ActivateEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Activate(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for DeactivateEvent {
	const DBUS_MEMBER: &'static str = "Deactivate";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for DeactivateEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Deactivate(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for RaiseEvent {
	const DBUS_MEMBER: &'static str = "Raise";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for RaiseEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Raise(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for LowerEvent {
	const DBUS_MEMBER: &'static str = "Lower";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for LowerEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Lower(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for MoveEvent {
	const DBUS_MEMBER: &'static str = "Move";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for MoveEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Move(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ResizeEvent {
	const DBUS_MEMBER: &'static str = "Resize";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for ResizeEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Resize(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ShadeEvent {
	const DBUS_MEMBER: &'static str = "Shade";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for ShadeEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Shade(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for UUshadeEvent {
	const DBUS_MEMBER: &'static str = "uUshade";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for UUshadeEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::UUshade(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for RestyleEvent {
	const DBUS_MEMBER: &'static str = "Restyle";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
	const REGISTRY_EVENT_STRING: &'static str = "Window:";

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
impl TryFrom<Event> for RestyleEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Window(WindowEvents::Restyle(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for WindowEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"PropertyChange" => Ok(WindowEvents::PropertyChange(ev.try_into()?)),
			"Minimize" => Ok(WindowEvents::Minimize(ev.try_into()?)),
			"Maximize" => Ok(WindowEvents::Maximize(ev.try_into()?)),
			"Restore" => Ok(WindowEvents::Restore(ev.try_into()?)),
			"Close" => Ok(WindowEvents::Close(ev.try_into()?)),
			"Create" => Ok(WindowEvents::Create(ev.try_into()?)),
			"Reparent" => Ok(WindowEvents::Reparent(ev.try_into()?)),
			"DesktopCreate" => Ok(WindowEvents::DesktopCreate(ev.try_into()?)),
			"DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(ev.try_into()?)),
			"Destroy" => Ok(WindowEvents::Destroy(ev.try_into()?)),
			"Activate" => Ok(WindowEvents::Activate(ev.try_into()?)),
			"Deactivate" => Ok(WindowEvents::Deactivate(ev.try_into()?)),
			"Raise" => Ok(WindowEvents::Raise(ev.try_into()?)),
			"Lower" => Ok(WindowEvents::Lower(ev.try_into()?)),
			"Move" => Ok(WindowEvents::Move(ev.try_into()?)),
			"Resize" => Ok(WindowEvents::Resize(ev.try_into()?)),
			"Shade" => Ok(WindowEvents::Shade(ev.try_into()?)),
			"uUshade" => Ok(WindowEvents::UUshade(ev.try_into()?)),
			"Restyle" => Ok(WindowEvents::Restyle(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
		}
	}
}

impl_event_conversions!(
	PropertyChangeEvent,
	WindowEvents,
	WindowEvents::PropertyChange,
	Event::Window
);
event_test_cases!(PropertyChangeEvent);
impl_to_dbus_message!(PropertyChangeEvent);
impl_from_dbus_message!(PropertyChangeEvent);
impl From<PropertyChangeEvent> for EventBodyOwned {
	fn from(event: PropertyChangeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.property,
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(MinimizeEvent, WindowEvents, WindowEvents::Minimize, Event::Window);
event_test_cases!(MinimizeEvent);
impl_to_dbus_message!(MinimizeEvent);
impl_from_dbus_message!(MinimizeEvent);
impl From<MinimizeEvent> for EventBodyOwned {
	fn from(_event: MinimizeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(MaximizeEvent, WindowEvents, WindowEvents::Maximize, Event::Window);
event_test_cases!(MaximizeEvent);
impl_to_dbus_message!(MaximizeEvent);
impl_from_dbus_message!(MaximizeEvent);
impl From<MaximizeEvent> for EventBodyOwned {
	fn from(_event: MaximizeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(RestoreEvent, WindowEvents, WindowEvents::Restore, Event::Window);
event_test_cases!(RestoreEvent);
impl_to_dbus_message!(RestoreEvent);
impl_from_dbus_message!(RestoreEvent);
impl From<RestoreEvent> for EventBodyOwned {
	fn from(_event: RestoreEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(CloseEvent, WindowEvents, WindowEvents::Close, Event::Window);
event_test_cases!(CloseEvent);
impl_to_dbus_message!(CloseEvent);
impl_from_dbus_message!(CloseEvent);
impl From<CloseEvent> for EventBodyOwned {
	fn from(_event: CloseEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(CreateEvent, WindowEvents, WindowEvents::Create, Event::Window);
event_test_cases!(CreateEvent);
impl_to_dbus_message!(CreateEvent);
impl_from_dbus_message!(CreateEvent);
impl From<CreateEvent> for EventBodyOwned {
	fn from(_event: CreateEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(ReparentEvent, WindowEvents, WindowEvents::Reparent, Event::Window);
event_test_cases!(ReparentEvent);
impl_to_dbus_message!(ReparentEvent);
impl_from_dbus_message!(ReparentEvent);
impl From<ReparentEvent> for EventBodyOwned {
	fn from(_event: ReparentEvent) -> Self {
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
	DesktopCreateEvent,
	WindowEvents,
	WindowEvents::DesktopCreate,
	Event::Window
);
event_test_cases!(DesktopCreateEvent);
impl_to_dbus_message!(DesktopCreateEvent);
impl_from_dbus_message!(DesktopCreateEvent);
impl From<DesktopCreateEvent> for EventBodyOwned {
	fn from(_event: DesktopCreateEvent) -> Self {
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
	DesktopDestroyEvent,
	WindowEvents,
	WindowEvents::DesktopDestroy,
	Event::Window
);
event_test_cases!(DesktopDestroyEvent);
impl_to_dbus_message!(DesktopDestroyEvent);
impl_from_dbus_message!(DesktopDestroyEvent);
impl From<DesktopDestroyEvent> for EventBodyOwned {
	fn from(_event: DesktopDestroyEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(DestroyEvent, WindowEvents, WindowEvents::Destroy, Event::Window);
event_test_cases!(DestroyEvent);
impl_to_dbus_message!(DestroyEvent);
impl_from_dbus_message!(DestroyEvent);
impl From<DestroyEvent> for EventBodyOwned {
	fn from(_event: DestroyEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(ActivateEvent, WindowEvents, WindowEvents::Activate, Event::Window);
event_test_cases!(ActivateEvent);
impl_to_dbus_message!(ActivateEvent);
impl_from_dbus_message!(ActivateEvent);
impl From<ActivateEvent> for EventBodyOwned {
	fn from(_event: ActivateEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(DeactivateEvent, WindowEvents, WindowEvents::Deactivate, Event::Window);
event_test_cases!(DeactivateEvent);
impl_to_dbus_message!(DeactivateEvent);
impl_from_dbus_message!(DeactivateEvent);
impl From<DeactivateEvent> for EventBodyOwned {
	fn from(_event: DeactivateEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(RaiseEvent, WindowEvents, WindowEvents::Raise, Event::Window);
event_test_cases!(RaiseEvent);
impl_to_dbus_message!(RaiseEvent);
impl_from_dbus_message!(RaiseEvent);
impl From<RaiseEvent> for EventBodyOwned {
	fn from(_event: RaiseEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(LowerEvent, WindowEvents, WindowEvents::Lower, Event::Window);
event_test_cases!(LowerEvent);
impl_to_dbus_message!(LowerEvent);
impl_from_dbus_message!(LowerEvent);
impl From<LowerEvent> for EventBodyOwned {
	fn from(_event: LowerEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(MoveEvent, WindowEvents, WindowEvents::Move, Event::Window);
event_test_cases!(MoveEvent);
impl_to_dbus_message!(MoveEvent);
impl_from_dbus_message!(MoveEvent);
impl From<MoveEvent> for EventBodyOwned {
	fn from(_event: MoveEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(ResizeEvent, WindowEvents, WindowEvents::Resize, Event::Window);
event_test_cases!(ResizeEvent);
impl_to_dbus_message!(ResizeEvent);
impl_from_dbus_message!(ResizeEvent);
impl From<ResizeEvent> for EventBodyOwned {
	fn from(_event: ResizeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(ShadeEvent, WindowEvents, WindowEvents::Shade, Event::Window);
event_test_cases!(ShadeEvent);
impl_to_dbus_message!(ShadeEvent);
impl_from_dbus_message!(ShadeEvent);
impl From<ShadeEvent> for EventBodyOwned {
	fn from(_event: ShadeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(UUshadeEvent, WindowEvents, WindowEvents::UUshade, Event::Window);
event_test_cases!(UUshadeEvent);
impl_to_dbus_message!(UUshadeEvent);
impl_from_dbus_message!(UUshadeEvent);
impl From<UUshadeEvent> for EventBodyOwned {
	fn from(_event: UUshadeEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(RestyleEvent, WindowEvents, WindowEvents::Restyle, Event::Window);
event_test_cases!(RestyleEvent);
impl_to_dbus_message!(RestyleEvent);
impl_from_dbus_message!(RestyleEvent);
impl From<RestyleEvent> for EventBodyOwned {
	fn from(_event: RestyleEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

/*impl HasMatchRule for PropertyChangeEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
}*/
/*impl HasMatchRule for MinimizeEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
}*/
/*impl HasMatchRule for MaximizeEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
}*/
/*impl HasMatchRule for RestoreEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
}*/
/*impl HasMatchRule for CloseEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
}*/
/*impl HasMatchRule for CreateEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
}*/
/*impl HasMatchRule for ReparentEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
}*/
/*impl HasMatchRule for DesktopCreateEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
}*/
/*impl HasMatchRule for DesktopDestroyEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
}*/
/*impl HasMatchRule for DestroyEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
}*/
/*impl HasMatchRule for ActivateEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
}*/
/*impl HasMatchRule for DeactivateEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
}*/
/*impl HasMatchRule for RaiseEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
}*/
/*impl HasMatchRule for LowerEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
}*/
/*impl HasMatchRule for MoveEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
}*/
/*impl HasMatchRule for ResizeEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
}*/
/*impl HasMatchRule for ShadeEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
}*/
/*impl HasMatchRule for UUshadeEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
}*/
/*impl HasMatchRule for RestyleEvent {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
}*/
/*impl HasRegistryEventString for PropertyChangeEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:PropertyChange";
}*/
/*impl HasRegistryEventString for MinimizeEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Minimize";
}*/
/*impl HasRegistryEventString for MaximizeEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Maximize";
}*/
/*impl HasRegistryEventString for RestoreEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Restore";
}*/
/*impl HasRegistryEventString for CloseEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Close";
}*/
/*impl HasRegistryEventString for CreateEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Create";
}*/
/*impl HasRegistryEventString for ReparentEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Reparent";
}*/
/*impl HasRegistryEventString for DesktopCreateEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:DesktopCreate";
}*/
/*impl HasRegistryEventString for DesktopDestroyEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:DesktopDestroy";
}*/
/*impl HasRegistryEventString for DestroyEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Destroy";
}*/
/*impl HasRegistryEventString for ActivateEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Activate";
}*/
/*impl HasRegistryEventString for DeactivateEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Deactivate";
}*/
/*impl HasRegistryEventString for RaiseEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Raise";
}*/
/*impl HasRegistryEventString for LowerEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Lower";
}*/
/*impl HasRegistryEventString for MoveEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Move";
}*/
/*impl HasRegistryEventString for ResizeEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Resize";
}*/
/*impl HasRegistryEventString for ShadeEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Shade";
}*/
/*impl HasRegistryEventString for UUshadeEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:uUshade";
}*/
/*impl HasRegistryEventString for RestyleEvent {
	const REGISTRY_EVENT_STRING: &'static str = "Window:Restyle";
}*/
impl HasRegistryEventString for WindowEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Window:";
}
