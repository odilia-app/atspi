use crate::AtspiError;

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod object {
	use crate::{
		error::AtspiError,
		events::{AnyEvent, EventInterfaces, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
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
	///          if let Event::Interfaces(EventInterfaces::Object(_event)) = ev {
	/// #            break;
	///              // do things with your event here
	///          }  else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum ObjectEvents {
		PropertyChange(PropertyChangeEvent),
		BoundsChanged(BoundsChangedEvent),
		LinkSelected(LinkSelectedEvent),
		StateChanged(StateChangedEvent),
		ChildrenChanged(ChildrenChangedEvent),
		VisibleDataChanged(VisibleDataChangedEvent),
		SelectionChanged(SelectionChangedEvent),
		ModelChanged(ModelChangedEvent),
		ActiveDescendantChanged(ActiveDescendantChangedEvent),
		Announcement(AnnouncementEvent),
		AttributesChanged(AttributesChangedEvent),
		RowInserted(RowInsertedEvent),
		RowReordered(RowReorderedEvent),
		RowDeleted(RowDeletedEvent),
		ColumnInserted(ColumnInsertedEvent),
		ColumnReordered(ColumnReorderedEvent),
		ColumnDeleted(ColumnDeletedEvent),
		TextBoundsChanged(TextBoundsChangedEvent),
		TextSelectionChanged(TextSelectionChangedEvent),
		TextChanged(TextChangedEvent),
		TextAttributesChanged(TextAttributesChangedEvent),
		TextCaretMoved(TextCaretMovedEvent),
	}

	impl HasMatchRule for ObjectEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object'";
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
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
	///         if let Ok(event) = PropertyChangeEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct PropertyChangeEvent {
		pub item: crate::events::Accessible,
		pub property: String,
		pub value: zbus::zvariant::OwnedValue,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::BoundsChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<BoundsChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("BoundsChanged")
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
	///         if let Ok(event) = BoundsChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct BoundsChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::LinkSelectedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LinkSelectedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("LinkSelected")
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
	///         if let Ok(event) = LinkSelectedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct LinkSelectedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::StateChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<StateChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("StateChanged")
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
	///         if let Ok(event) = StateChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct StateChangedEvent {
		pub item: crate::events::Accessible,
		pub state: String,
		pub enabled: i32,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::ChildrenChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ChildrenChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("ChildrenChanged")
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
	///         if let Ok(event) = ChildrenChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ChildrenChangedEvent {
		pub item: crate::events::Accessible,
		pub operation: String,
		pub index_in_parent: i32,
		pub child: zbus::zvariant::OwnedValue,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::VisibleDataChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<VisibleDataChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("VisibleDataChanged")
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
	///         if let Ok(event) = VisibleDataChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct VisibleDataChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::SelectionChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<SelectionChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("SelectionChanged")
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
	///         if let Ok(event) = SelectionChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct SelectionChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::ModelChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ModelChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("ModelChanged")
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
	///         if let Ok(event) = ModelChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ModelChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::ActiveDescendantChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ActiveDescendantChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("ActiveDescendantChanged")
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
	///         if let Ok(event) = ActiveDescendantChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ActiveDescendantChangedEvent {
		pub item: crate::events::Accessible,
		pub child: zbus::zvariant::OwnedValue,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::AnnouncementEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AnnouncementEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("Announcement")
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
	///         if let Ok(event) = AnnouncementEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct AnnouncementEvent {
		pub item: crate::events::Accessible,
		pub text: String,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::AttributesChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AttributesChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("AttributesChanged")
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
	///         if let Ok(event) = AttributesChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct AttributesChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::RowInsertedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RowInsertedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("RowInserted")
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
	///         if let Ok(event) = RowInsertedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct RowInsertedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::RowReorderedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RowReorderedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("RowReordered")
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
	///         if let Ok(event) = RowReorderedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct RowReorderedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::RowDeletedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RowDeletedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("RowDeleted")
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
	///         if let Ok(event) = RowDeletedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct RowDeletedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::ColumnInsertedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnInsertedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("ColumnInserted")
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
	///         if let Ok(event) = ColumnInsertedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ColumnInsertedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::ColumnReorderedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnReorderedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("ColumnReordered")
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
	///         if let Ok(event) = ColumnReorderedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ColumnReorderedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::ColumnDeletedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnDeletedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("ColumnDeleted")
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
	///         if let Ok(event) = ColumnDeletedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ColumnDeletedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::TextBoundsChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextBoundsChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("TextBoundsChanged")
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
	///         if let Ok(event) = TextBoundsChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct TextBoundsChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::TextSelectionChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextSelectionChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("TextSelectionChanged")
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
	///         if let Ok(event) = TextSelectionChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct TextSelectionChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::TextChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("TextChanged")
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
	///         if let Ok(event) = TextChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct TextChangedEvent {
		pub item: crate::events::Accessible,
		pub detail: String,
		pub start_pos: i32,
		pub length: i32,
		pub text: zbus::zvariant::OwnedValue,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::TextAttributesChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextAttributesChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("TextAttributesChanged")
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
	///         if let Ok(event) = TextAttributesChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct TextAttributesChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::object::TextCaretMovedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextCaretMovedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("TextCaretMoved")
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
	///         if let Ok(event) = TextCaretMovedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct TextCaretMovedEvent {
		pub item: crate::events::Accessible,
		pub position: i32,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
	}

	impl GenericEvent for PropertyChangeEvent {
		const DBUS_MEMBER: &'static str = "PropertyChange";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for PropertyChangeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::PropertyChange(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for BoundsChangedEvent {
		const DBUS_MEMBER: &'static str = "BoundsChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='BoundsChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for BoundsChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::BoundsChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for LinkSelectedEvent {
		const DBUS_MEMBER: &'static str = "LinkSelected";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='LinkSelected'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for LinkSelectedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::LinkSelected(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for StateChangedEvent {
		const DBUS_MEMBER: &'static str = "StateChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for StateChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::StateChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ChildrenChangedEvent {
		const DBUS_MEMBER: &'static str = "ChildrenChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ChildrenChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ChildrenChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ChildrenChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for VisibleDataChangedEvent {
		const DBUS_MEMBER: &'static str = "VisibleDataChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='VisibleDataChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for VisibleDataChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::VisibleDataChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for SelectionChangedEvent {
		const DBUS_MEMBER: &'static str = "SelectionChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='SelectionChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for SelectionChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::SelectionChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ModelChangedEvent {
		const DBUS_MEMBER: &'static str = "ModelChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ModelChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ModelChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ModelChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ActiveDescendantChangedEvent {
		const DBUS_MEMBER: &'static str = "ActiveDescendantChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ActiveDescendantChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ActiveDescendantChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ActiveDescendantChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for AnnouncementEvent {
		const DBUS_MEMBER: &'static str = "Announcement";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for AnnouncementEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::Announcement(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for AttributesChangedEvent {
		const DBUS_MEMBER: &'static str = "AttributesChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for AttributesChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::AttributesChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for RowInsertedEvent {
		const DBUS_MEMBER: &'static str = "RowInserted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='RowInserted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for RowInsertedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::RowInserted(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for RowReorderedEvent {
		const DBUS_MEMBER: &'static str = "RowReordered";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='RowReordered'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for RowReorderedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::RowReordered(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for RowDeletedEvent {
		const DBUS_MEMBER: &'static str = "RowDeleted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='RowDeleted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for RowDeletedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::RowDeleted(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ColumnInsertedEvent {
		const DBUS_MEMBER: &'static str = "ColumnInserted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnInserted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ColumnInsertedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ColumnInserted(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ColumnReorderedEvent {
		const DBUS_MEMBER: &'static str = "ColumnReordered";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnReordered'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ColumnReorderedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ColumnReordered(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ColumnDeletedEvent {
		const DBUS_MEMBER: &'static str = "ColumnDeleted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnDeleted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ColumnDeletedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::ColumnDeleted(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for TextBoundsChangedEvent {
		const DBUS_MEMBER: &'static str = "TextBoundsChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextBoundsChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for TextBoundsChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextBoundsChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for TextSelectionChangedEvent {
		const DBUS_MEMBER: &'static str = "TextSelectionChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextSelectionChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for TextSelectionChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextSelectionChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for TextChangedEvent {
		const DBUS_MEMBER: &'static str = "TextChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for TextChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for TextAttributesChangedEvent {
		const DBUS_MEMBER: &'static str = "TextAttributesChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextAttributesChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for TextAttributesChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextAttributesChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for TextCaretMovedEvent {
		const DBUS_MEMBER: &'static str = "TextCaretMoved";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for TextCaretMovedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextCaretMoved(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl TryFrom<AnyEvent> for ObjectEvents {
		type Error = AtspiError;

		fn try_from(ev: AnyEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"PropertyChange" => Ok(ObjectEvents::PropertyChange(PropertyChangeEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					property: ev.body.kind,
					value: ev.body.any_data,
					properties: ev.body.properties,
				})),
				"BoundsChanged" => Ok(ObjectEvents::BoundsChanged(BoundsChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"LinkSelected" => Ok(ObjectEvents::LinkSelected(LinkSelectedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"StateChanged" => Ok(ObjectEvents::StateChanged(StateChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					state: ev.body.kind,
					enabled: ev.body.detail1,
					properties: ev.body.properties,
				})),
				"ChildrenChanged" => Ok(ObjectEvents::ChildrenChanged(ChildrenChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					operation: ev.body.kind,
					index_in_parent: ev.body.detail1,
					child: ev.body.any_data,
					properties: ev.body.properties,
				})),
				"VisibleDataChanged" => {
					Ok(ObjectEvents::VisibleDataChanged(VisibleDataChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"SelectionChanged" => Ok(ObjectEvents::SelectionChanged(SelectionChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ModelChanged" => Ok(ObjectEvents::ModelChanged(ModelChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ActiveDescendantChanged" => {
					Ok(ObjectEvents::ActiveDescendantChanged(ActiveDescendantChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						child: ev.body.any_data,
						properties: ev.body.properties,
					}))
				}
				"Announcement" => Ok(ObjectEvents::Announcement(AnnouncementEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					text: ev.body.kind,
					properties: ev.body.properties,
				})),
				"AttributesChanged" => {
					Ok(ObjectEvents::AttributesChanged(AttributesChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"RowInserted" => Ok(ObjectEvents::RowInserted(RowInsertedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"RowReordered" => Ok(ObjectEvents::RowReordered(RowReorderedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"RowDeleted" => Ok(ObjectEvents::RowDeleted(RowDeletedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ColumnInserted" => Ok(ObjectEvents::ColumnInserted(ColumnInsertedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ColumnReordered" => Ok(ObjectEvents::ColumnReordered(ColumnReorderedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ColumnDeleted" => Ok(ObjectEvents::ColumnDeleted(ColumnDeletedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"TextBoundsChanged" => {
					Ok(ObjectEvents::TextBoundsChanged(TextBoundsChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"TextSelectionChanged" => {
					Ok(ObjectEvents::TextSelectionChanged(TextSelectionChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"TextChanged" => Ok(ObjectEvents::TextChanged(TextChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					detail: ev.body.kind,
					start_pos: ev.body.detail1,
					length: ev.body.detail2,
					text: ev.body.any_data,
					properties: ev.body.properties,
				})),
				"TextAttributesChanged" => {
					Ok(ObjectEvents::TextAttributesChanged(TextAttributesChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"TextCaretMoved" => Ok(ObjectEvents::TextCaretMoved(TextCaretMovedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					position: ev.body.detail1,
					properties: ev.body.properties,
				})),
				_ => Err(AtspiError::MemberMatch("No matching member for Object".into())),
			}
		}
	}

	/*impl HasMatchRule for PropertyChangeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
	}*/
	/*impl HasMatchRule for BoundsChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='BoundsChanged'";
	}*/
	/*impl HasMatchRule for LinkSelectedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='LinkSelected'";
	}*/
	/*impl HasMatchRule for StateChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'";
	}*/
	/*impl HasMatchRule for ChildrenChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ChildrenChanged'";
	}*/
	/*impl HasMatchRule for VisibleDataChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='VisibleDataChanged'";
	}*/
	/*impl HasMatchRule for SelectionChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='SelectionChanged'";
	}*/
	/*impl HasMatchRule for ModelChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ModelChanged'";
	}*/
	/*impl HasMatchRule for ActiveDescendantChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ActiveDescendantChanged'";
	}*/
	/*impl HasMatchRule for AnnouncementEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'";
	}*/
	/*impl HasMatchRule for AttributesChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'";
	}*/
	/*impl HasMatchRule for RowInsertedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowInserted'";
	}*/
	/*impl HasMatchRule for RowReorderedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowReordered'";
	}*/
	/*impl HasMatchRule for RowDeletedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowDeleted'";
	}*/
	/*impl HasMatchRule for ColumnInsertedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnInserted'";
	}*/
	/*impl HasMatchRule for ColumnReorderedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnReordered'";
	}*/
	/*impl HasMatchRule for ColumnDeletedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnDeleted'";
	}*/
	/*impl HasMatchRule for TextBoundsChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextBoundsChanged'";
	}*/
	/*impl HasMatchRule for TextSelectionChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextSelectionChanged'";
	}*/
	/*impl HasMatchRule for TextChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextChanged'";
	}*/
	/*impl HasMatchRule for TextAttributesChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextAttributesChanged'";
	}*/
	/*impl HasMatchRule for TextCaretMovedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved'";
	}*/
	/*impl HasRegistryEventString for PropertyChangeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:PropertyChange";
	}*/
	/*impl HasRegistryEventString for BoundsChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:BoundsChanged";
	}*/
	/*impl HasRegistryEventString for LinkSelectedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:LinkSelected";
	}*/
	/*impl HasRegistryEventString for StateChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:StateChanged";
	}*/
	/*impl HasRegistryEventString for ChildrenChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ChildrenChanged";
	}*/
	/*impl HasRegistryEventString for VisibleDataChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:VisibleDataChanged";
	}*/
	/*impl HasRegistryEventString for SelectionChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:SelectionChanged";
	}*/
	/*impl HasRegistryEventString for ModelChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ModelChanged";
	}*/
	/*impl HasRegistryEventString for ActiveDescendantChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ActiveDescendantChanged";
	}*/
	/*impl HasRegistryEventString for AnnouncementEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:Announcement";
	}*/
	/*impl HasRegistryEventString for AttributesChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:AttributesChanged";
	}*/
	/*impl HasRegistryEventString for RowInsertedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:RowInserted";
	}*/
	/*impl HasRegistryEventString for RowReorderedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:RowReordered";
	}*/
	/*impl HasRegistryEventString for RowDeletedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:RowDeleted";
	}*/
	/*impl HasRegistryEventString for ColumnInsertedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ColumnInserted";
	}*/
	/*impl HasRegistryEventString for ColumnReorderedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ColumnReordered";
	}*/
	/*impl HasRegistryEventString for ColumnDeletedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ColumnDeleted";
	}*/
	/*impl HasRegistryEventString for TextBoundsChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextBoundsChanged";
	}*/
	/*impl HasRegistryEventString for TextSelectionChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextSelectionChanged";
	}*/
	/*impl HasRegistryEventString for TextChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextChanged";
	}*/
	/*impl HasRegistryEventString for TextAttributesChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextAttributesChanged";
	}*/
	/*impl HasRegistryEventString for TextCaretMovedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextCaretMoved";
	}*/
	impl HasRegistryEventString for ObjectEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod window {
	use crate::{
		error::AtspiError,
		events::{AnyEvent, EventInterfaces, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
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
	///          if let Event::Interfaces(EventInterfaces::Window(_event)) = ev {
	/// #            break;
	///              // do things with your event here
	///          }  else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug)]
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

	impl HasMatchRule for WindowEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window'";
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
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
	///         if let Ok(event) = PropertyChangeEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct PropertyChangeEvent {
		pub item: crate::events::Accessible,
		pub property: String,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::MinimizeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<MinimizeEvent>().await.unwrap();
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
	/// #       .arg("Minimize")
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
	///         if let Ok(event) = MinimizeEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct MinimizeEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::MaximizeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<MaximizeEvent>().await.unwrap();
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
	/// #       .arg("Maximize")
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
	///         if let Ok(event) = MaximizeEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct MaximizeEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::RestoreEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RestoreEvent>().await.unwrap();
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
	/// #       .arg("Restore")
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
	///         if let Ok(event) = RestoreEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct RestoreEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::CloseEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<CloseEvent>().await.unwrap();
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
	/// #       .arg("Close")
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
	///         if let Ok(event) = CloseEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct CloseEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::CreateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<CreateEvent>().await.unwrap();
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
	/// #       .arg("Create")
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
	///         if let Ok(event) = CreateEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct CreateEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::ReparentEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ReparentEvent>().await.unwrap();
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
	/// #       .arg("Reparent")
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
	///         if let Ok(event) = ReparentEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ReparentEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::DesktopCreateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DesktopCreateEvent>().await.unwrap();
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
	/// #       .arg("DesktopCreate")
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
	///         if let Ok(event) = DesktopCreateEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct DesktopCreateEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::DesktopDestroyEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DesktopDestroyEvent>().await.unwrap();
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
	/// #       .arg("DesktopDestroy")
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
	///         if let Ok(event) = DesktopDestroyEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct DesktopDestroyEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::DestroyEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DestroyEvent>().await.unwrap();
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
	/// #       .arg("Destroy")
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
	///         if let Ok(event) = DestroyEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct DestroyEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::ActivateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ActivateEvent>().await.unwrap();
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
	/// #       .arg("Activate")
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
	///         if let Ok(event) = ActivateEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ActivateEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::DeactivateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DeactivateEvent>().await.unwrap();
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
	/// #       .arg("Deactivate")
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
	///         if let Ok(event) = DeactivateEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct DeactivateEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::RaiseEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RaiseEvent>().await.unwrap();
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
	/// #       .arg("Raise")
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
	///         if let Ok(event) = RaiseEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct RaiseEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::LowerEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LowerEvent>().await.unwrap();
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
	/// #       .arg("Lower")
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
	///         if let Ok(event) = LowerEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct LowerEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::MoveEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<MoveEvent>().await.unwrap();
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
	/// #       .arg("Move")
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
	///         if let Ok(event) = MoveEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct MoveEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::ResizeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ResizeEvent>().await.unwrap();
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
	/// #       .arg("Resize")
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
	///         if let Ok(event) = ResizeEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ResizeEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::ShadeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ShadeEvent>().await.unwrap();
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
	/// #       .arg("Shade")
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
	///         if let Ok(event) = ShadeEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ShadeEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::UUshadeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<UUshadeEvent>().await.unwrap();
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
	/// #       .arg("uUshade")
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
	///         if let Ok(event) = UUshadeEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct UUshadeEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::window::RestyleEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RestyleEvent>().await.unwrap();
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
	/// #       .arg("Restyle")
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
	///         if let Ok(event) = RestyleEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct RestyleEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
	}

	impl GenericEvent for PropertyChangeEvent {
		const DBUS_MEMBER: &'static str = "PropertyChange";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for PropertyChangeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::PropertyChange(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for MinimizeEvent {
		const DBUS_MEMBER: &'static str = "Minimize";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for MinimizeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Minimize(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for MaximizeEvent {
		const DBUS_MEMBER: &'static str = "Maximize";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for MaximizeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Maximize(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for RestoreEvent {
		const DBUS_MEMBER: &'static str = "Restore";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for RestoreEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Restore(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for CloseEvent {
		const DBUS_MEMBER: &'static str = "Close";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for CloseEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Close(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for CreateEvent {
		const DBUS_MEMBER: &'static str = "Create";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for CreateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Create(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ReparentEvent {
		const DBUS_MEMBER: &'static str = "Reparent";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ReparentEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Reparent(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for DesktopCreateEvent {
		const DBUS_MEMBER: &'static str = "DesktopCreate";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for DesktopCreateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::DesktopCreate(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for DesktopDestroyEvent {
		const DBUS_MEMBER: &'static str = "DesktopDestroy";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for DesktopDestroyEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::DesktopDestroy(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for DestroyEvent {
		const DBUS_MEMBER: &'static str = "Destroy";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for DestroyEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Destroy(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ActivateEvent {
		const DBUS_MEMBER: &'static str = "Activate";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ActivateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Activate(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for DeactivateEvent {
		const DBUS_MEMBER: &'static str = "Deactivate";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for DeactivateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Deactivate(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for RaiseEvent {
		const DBUS_MEMBER: &'static str = "Raise";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for RaiseEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Raise(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for LowerEvent {
		const DBUS_MEMBER: &'static str = "Lower";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for LowerEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Lower(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for MoveEvent {
		const DBUS_MEMBER: &'static str = "Move";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for MoveEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Move(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ResizeEvent {
		const DBUS_MEMBER: &'static str = "Resize";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ResizeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Resize(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ShadeEvent {
		const DBUS_MEMBER: &'static str = "Shade";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ShadeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Shade(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for UUshadeEvent {
		const DBUS_MEMBER: &'static str = "uUshade";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for UUshadeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::UUshade(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for RestyleEvent {
		const DBUS_MEMBER: &'static str = "Restyle";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for RestyleEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Window(WindowEvents::Restyle(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl TryFrom<AnyEvent> for WindowEvents {
		type Error = AtspiError;

		fn try_from(ev: AnyEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"PropertyChange" => Ok(WindowEvents::PropertyChange(PropertyChangeEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					property: ev.body.kind,
					properties: ev.body.properties,
				})),
				"Minimize" => Ok(WindowEvents::Minimize(MinimizeEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Maximize" => Ok(WindowEvents::Maximize(MaximizeEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Restore" => Ok(WindowEvents::Restore(RestoreEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Close" => Ok(WindowEvents::Close(CloseEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Create" => Ok(WindowEvents::Create(CreateEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Reparent" => Ok(WindowEvents::Reparent(ReparentEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"DesktopCreate" => Ok(WindowEvents::DesktopCreate(DesktopCreateEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(DesktopDestroyEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Destroy" => Ok(WindowEvents::Destroy(DestroyEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Activate" => Ok(WindowEvents::Activate(ActivateEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Deactivate" => Ok(WindowEvents::Deactivate(DeactivateEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Raise" => Ok(WindowEvents::Raise(RaiseEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Lower" => Ok(WindowEvents::Lower(LowerEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Move" => Ok(WindowEvents::Move(MoveEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Resize" => Ok(WindowEvents::Resize(ResizeEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Shade" => Ok(WindowEvents::Shade(ShadeEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"uUshade" => Ok(WindowEvents::UUshade(UUshadeEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Restyle" => Ok(WindowEvents::Restyle(RestyleEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
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
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod mouse {
	use crate::{
		error::AtspiError,
		events::{AnyEvent, EventInterfaces, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::mouse::AbsEvent;
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
	///          if let Event::Interfaces(EventInterfaces::Mouse(_event)) = ev {
	/// #            break;
	///              // do things with your event here
	///          }  else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum MouseEvents {
		Abs(AbsEvent),
		Rel(RelEvent),
		Button(ButtonEvent),
	}

	impl HasMatchRule for MouseEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse'";
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::mouse::AbsEvent;
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
	///         if let Ok(event) = AbsEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct AbsEvent {
		pub item: crate::events::Accessible,
		pub x: i32,
		pub y: i32,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::mouse::RelEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RelEvent>().await.unwrap();
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
	/// #       .arg("Rel")
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
	///         if let Ok(event) = RelEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct RelEvent {
		pub item: crate::events::Accessible,
		pub x: i32,
		pub y: i32,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::mouse::ButtonEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ButtonEvent>().await.unwrap();
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
	/// #       .arg("Button")
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
	///         if let Ok(event) = ButtonEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ButtonEvent {
		pub item: crate::events::Accessible,
		pub detail: String,
		pub mouse_x: i32,
		pub mouse_y: i32,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
	}

	impl GenericEvent for AbsEvent {
		const DBUS_MEMBER: &'static str = "Abs";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for AbsEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Mouse(MouseEvents::Abs(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for RelEvent {
		const DBUS_MEMBER: &'static str = "Rel";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for RelEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Mouse(MouseEvents::Rel(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ButtonEvent {
		const DBUS_MEMBER: &'static str = "Button";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ButtonEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Mouse(MouseEvents::Button(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl TryFrom<AnyEvent> for MouseEvents {
		type Error = AtspiError;

		fn try_from(ev: AnyEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Abs" => Ok(MouseEvents::Abs(AbsEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					x: ev.body.detail1,
					y: ev.body.detail2,
					properties: ev.body.properties,
				})),
				"Rel" => Ok(MouseEvents::Rel(RelEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					x: ev.body.detail1,
					y: ev.body.detail2,
					properties: ev.body.properties,
				})),
				"Button" => Ok(MouseEvents::Button(ButtonEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					detail: ev.body.kind,
					mouse_x: ev.body.detail1,
					mouse_y: ev.body.detail2,
					properties: ev.body.properties,
				})),
				_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
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
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod keyboard {
	use crate::{
		error::AtspiError,
		events::{AnyEvent, EventInterfaces, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::keyboard::ModifiersEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ModifiersEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Keyboard")
	/// #       .arg("Modifiers")
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
	///          if let Event::Interfaces(EventInterfaces::Keyboard(_event)) = ev {
	/// #            break;
	///              // do things with your event here
	///          }  else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum KeyboardEvents {
		Modifiers(ModifiersEvent),
	}

	impl HasMatchRule for KeyboardEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Keyboard'";
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::keyboard::ModifiersEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ModifiersEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Keyboard")
	/// #       .arg("Modifiers")
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
	///         if let Ok(event) = ModifiersEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ModifiersEvent {
		pub item: crate::events::Accessible,
		pub previous_modifiers: i32,
		pub current_modifiers: i32,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
	}

	impl GenericEvent for ModifiersEvent {
		const DBUS_MEMBER: &'static str = "Modifiers";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
		const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ModifiersEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Keyboard(KeyboardEvents::Modifiers(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl TryFrom<AnyEvent> for KeyboardEvents {
		type Error = AtspiError;

		fn try_from(ev: AnyEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Modifiers" => Ok(KeyboardEvents::Modifiers(ModifiersEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					previous_modifiers: ev.body.detail1,
					current_modifiers: ev.body.detail2,
					properties: ev.body.properties,
				})),
				_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
			}
		}
	}

	/*impl HasMatchRule for ModifiersEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
	}*/
	/*impl HasRegistryEventString for ModifiersEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Keyboard:Modifiers";
	}*/
	impl HasRegistryEventString for KeyboardEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod terminal {
	use crate::{
		error::AtspiError,
		events::{AnyEvent, EventInterfaces, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::terminal::LineChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LineChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Terminal")
	/// #       .arg("LineChanged")
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
	///          if let Event::Interfaces(EventInterfaces::Terminal(_event)) = ev {
	/// #            break;
	///              // do things with your event here
	///          }  else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum TerminalEvents {
		LineChanged(LineChangedEvent),
		ColumnCountChanged(ColumnCountChangedEvent),
		LineCountChanged(LineCountChangedEvent),
		ApplicationChanged(ApplicationChangedEvent),
		CharWidthChanged(CharWidthChangedEvent),
	}

	impl HasMatchRule for TerminalEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal'";
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::terminal::LineChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LineChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Terminal")
	/// #       .arg("LineChanged")
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
	///         if let Ok(event) = LineChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct LineChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::terminal::ColumnCountChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnCountChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Terminal")
	/// #       .arg("ColumncountChanged")
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
	///         if let Ok(event) = ColumnCountChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ColumnCountChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::terminal::LineCountChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LineCountChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Terminal")
	/// #       .arg("LinecountChanged")
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
	///         if let Ok(event) = LineCountChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct LineCountChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::terminal::ApplicationChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ApplicationChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Terminal")
	/// #       .arg("ApplicationChanged")
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
	///         if let Ok(event) = ApplicationChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ApplicationChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::terminal::CharWidthChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<CharWidthChangedEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Terminal")
	/// #       .arg("CharwidthChanged")
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
	///         if let Ok(event) = CharWidthChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct CharWidthChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
	}

	impl GenericEvent for LineChangedEvent {
		const DBUS_MEMBER: &'static str = "LineChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for LineChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::LineChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ColumnCountChangedEvent {
		const DBUS_MEMBER: &'static str = "ColumncountChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ColumnCountChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::ColumnCountChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for LineCountChangedEvent {
		const DBUS_MEMBER: &'static str = "LinecountChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for LineCountChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::LineCountChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ApplicationChangedEvent {
		const DBUS_MEMBER: &'static str = "ApplicationChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ApplicationChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::ApplicationChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for CharWidthChangedEvent {
		const DBUS_MEMBER: &'static str = "CharwidthChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for CharWidthChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Terminal(TerminalEvents::CharWidthChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl TryFrom<AnyEvent> for TerminalEvents {
		type Error = AtspiError;

		fn try_from(ev: AnyEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LineChanged" => Ok(TerminalEvents::LineChanged(LineChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ColumncountChanged" => {
					Ok(TerminalEvents::ColumnCountChanged(ColumnCountChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"LinecountChanged" => Ok(TerminalEvents::LineCountChanged(LineCountChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ApplicationChanged" => {
					Ok(TerminalEvents::ApplicationChanged(ApplicationChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"CharwidthChanged" => Ok(TerminalEvents::CharWidthChanged(CharWidthChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
			}
		}
	}

	/*impl HasMatchRule for LineChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
	}*/
	/*impl HasMatchRule for ColumnCountChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
	}*/
	/*impl HasMatchRule for LineCountChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
	}*/
	/*impl HasMatchRule for ApplicationChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
	}*/
	/*impl HasMatchRule for CharWidthChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
	}*/
	/*impl HasRegistryEventString for LineChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:LineChanged";
	}*/
	/*impl HasRegistryEventString for ColumnCountChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:ColumncountChanged";
	}*/
	/*impl HasRegistryEventString for LineCountChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:LinecountChanged";
	}*/
	/*impl HasRegistryEventString for ApplicationChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:ApplicationChanged";
	}*/
	/*impl HasRegistryEventString for CharWidthChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:CharwidthChanged";
	}*/
	impl HasRegistryEventString for TerminalEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod document {
	use crate::{
		error::AtspiError,
		events::{AnyEvent, EventInterfaces, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::document::LoadCompleteEvent;
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
	///          if let Event::Interfaces(EventInterfaces::Document(_event)) = ev {
	/// #            break;
	///              // do things with your event here
	///          }  else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum DocumentEvents {
		LoadComplete(LoadCompleteEvent),
		Reload(ReloadEvent),
		LoadStopped(LoadStoppedEvent),
		ContentChanged(ContentChangedEvent),
		AttributesChanged(AttributesChangedEvent),
		PageChanged(PageChangedEvent),
	}

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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::document::LoadCompleteEvent;
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
	///         if let Ok(event) = LoadCompleteEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct LoadCompleteEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::document::ReloadEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ReloadEvent>().await.unwrap();
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
	/// #       .arg("Reload")
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
	///         if let Ok(event) = ReloadEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ReloadEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::document::LoadStoppedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LoadStoppedEvent>().await.unwrap();
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
	/// #       .arg("LoadStopped")
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
	///         if let Ok(event) = LoadStoppedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct LoadStoppedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::document::ContentChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ContentChangedEvent>().await.unwrap();
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
	/// #       .arg("ContentChanged")
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
	///         if let Ok(event) = ContentChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct ContentChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::document::AttributesChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AttributesChangedEvent>().await.unwrap();
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
	/// #       .arg("AttributesChanged")
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
	///         if let Ok(event) = AttributesChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct AttributesChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::document::PageChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<PageChangedEvent>().await.unwrap();
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
	/// #       .arg("PageChanged")
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
	///         if let Ok(event) = PageChangedEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct PageChangedEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
	}

	impl GenericEvent for LoadCompleteEvent {
		const DBUS_MEMBER: &'static str = "LoadComplete";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for LoadCompleteEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::LoadComplete(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ReloadEvent {
		const DBUS_MEMBER: &'static str = "Reload";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='Reload'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ReloadEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::Reload(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for LoadStoppedEvent {
		const DBUS_MEMBER: &'static str = "LoadStopped";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for LoadStoppedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::LoadStopped(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for ContentChangedEvent {
		const DBUS_MEMBER: &'static str = "ContentChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for ContentChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::ContentChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for AttributesChangedEvent {
		const DBUS_MEMBER: &'static str = "AttributesChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for AttributesChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::AttributesChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl GenericEvent for PageChangedEvent {
		const DBUS_MEMBER: &'static str = "PageChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for PageChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Document(DocumentEvents::PageChanged(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl TryFrom<AnyEvent> for DocumentEvents {
		type Error = AtspiError;

		fn try_from(ev: AnyEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LoadComplete" => Ok(DocumentEvents::LoadComplete(LoadCompleteEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"Reload" => Ok(DocumentEvents::Reload(ReloadEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"LoadStopped" => Ok(DocumentEvents::LoadStopped(LoadStoppedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"ContentChanged" => Ok(DocumentEvents::ContentChanged(ContentChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				"AttributesChanged" => {
					Ok(DocumentEvents::AttributesChanged(AttributesChangedEvent {
						item: crate::events::Accessible {
							name: ev
								.message
								.header()
								.unwrap()
								.sender()
								.unwrap()
								.unwrap()
								.to_owned()
								.into(),
							path: ev.message.path().unwrap().into(),
						},
						properties: ev.body.properties,
					}))
				}
				"PageChanged" => Ok(DocumentEvents::PageChanged(PageChangedEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
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
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod focus {
	use crate::{
		error::AtspiError,
		events::{AnyEvent, EventInterfaces, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::focus::FocusEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<FocusEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Focus")
	/// #       .arg("Focus")
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
	///          if let Event::Interfaces(EventInterfaces::Focus(_event)) = ev {
	/// #            break;
	///              // do things with your event here
	///          }  else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum FocusEvents {
		Focus(FocusEvent),
	}

	impl HasMatchRule for FocusEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Focus'";
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
	/// use atspi::{events::EventInterfaces, Event};
	/// use atspi::identify::focus::FocusEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<FocusEvent>().await.unwrap();
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
	/// #       .arg("org.a11y.atspi.Event.Focus")
	/// #       .arg("Focus")
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
	///         if let Ok(event) = FocusEvent::try_from(ev) {
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone)]
	pub struct FocusEvent {
		pub item: crate::events::Accessible,
		pub properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
	}

	impl GenericEvent for FocusEvent {
		const DBUS_MEMBER: &'static str = "Focus";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
		const REGISTRY_EVENT_STRING: &'static str = "Focus:";
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
	}
	#[rustfmt::skip]
    impl TryFrom<Event> for FocusEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Interfaces(EventInterfaces::Focus(FocusEvents::Focus(inner_event))) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

	impl TryFrom<AnyEvent> for FocusEvents {
		type Error = AtspiError;

		fn try_from(ev: AnyEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Focus" => Ok(FocusEvents::Focus(FocusEvent {
					item: crate::events::Accessible {
						name: ev
							.message
							.header()
							.unwrap()
							.sender()
							.unwrap()
							.unwrap()
							.to_owned()
							.into(),
						path: ev.message.path().unwrap().into(),
					},
					properties: ev.body.properties,
				})),
				_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
			}
		}
	}

	/*impl HasMatchRule for FocusEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
	}*/
	/*impl HasRegistryEventString for FocusEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Focus:Focus";
	}*/
	impl HasRegistryEventString for FocusEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Focus:";
	}
}
use crate::events::{AddAccessibleEvent, CacheEvents, RemoveAccessibleEvent};
use crate::Event;
#[rustfmt::skip]
    impl TryFrom<Event> for AddAccessibleEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Cache(CacheEvents::Add(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

#[rustfmt::skip]
    impl TryFrom<Event> for RemoveAccessibleEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Cache(CacheEvents::Remove(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

use crate::events::{
	EventListenerDeregisteredEvent, EventListenerEvents, EventListenerRegisteredEvent,
};
#[rustfmt::skip]
    impl TryFrom<Event> for EventListenerRegisteredEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Listener(EventListenerEvents::Registered(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

#[rustfmt::skip]
    impl TryFrom<Event> for EventListenerDeregisteredEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Listener(EventListenerEvents::Deregistered(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}

use crate::events::AvailableEvent;
#[rustfmt::skip]
    impl TryFrom<Event> for AvailableEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Available(inner_event) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
