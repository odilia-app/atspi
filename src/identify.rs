
    use crate::AtspiError;
    

    
#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod object {
	use atspi_macros::TrySignify;
	use crate::{
        Event,
		error::AtspiError,
		events::{AtspiEvent, GenericEvent, EventInterfaces},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
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
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///          let Event::Interfaces(EventInterfaces::Object(_event)) = ev else { continue };
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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = PropertyChangeEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct PropertyChangeEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = BoundsChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct BoundsChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = LinkSelectedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LinkSelectedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = StateChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct StateChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ChildrenChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ChildrenChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = VisibleDataChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct VisibleDataChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = SelectionChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct SelectionChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ModelChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ModelChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ActiveDescendantChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ActiveDescendantChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = AnnouncementEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AnnouncementEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = AttributesChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AttributesChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = RowInsertedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RowInsertedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = RowReorderedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RowReorderedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = RowDeletedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RowDeletedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ColumnInsertedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnInsertedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ColumnReorderedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnReorderedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ColumnDeletedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnDeletedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = TextBoundsChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextBoundsChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = TextSelectionChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextSelectionChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = TextChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = TextAttributesChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextAttributesChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = TextCaretMovedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextCaretMovedEvent(pub(crate) AtspiEvent);
	
	
    
	impl PropertyChangeEvent {
		
		#[must_use]
		pub fn value(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}

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
    

    
	impl StateChangedEvent {
		
		#[must_use]
		pub fn enabled(&self) -> i32 {
			self.0.detail1()
		}
	
	}

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
    

    
	impl ChildrenChangedEvent {
		
		#[must_use]
		pub fn index_in_parent(&self) -> i32 {
			self.0.detail1()
		}
	


		#[must_use]
		pub fn child(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}

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
    

    
	impl ActiveDescendantChangedEvent {
		
		#[must_use]
		pub fn child(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}

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
    

    
	impl TextChangedEvent {
		
		#[must_use]
		pub fn start_pos(&self) -> i32 {
			self.0.detail1()
		}
	


		#[must_use]
		pub fn end_pos(&self) -> i32 {
			self.0.detail2()
		}
	


		#[must_use]
		pub fn text(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}

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
    

    
	impl TextCaretMovedEvent {
		
		#[must_use]
		pub fn position(&self) -> i32 {
			self.0.detail1()
		}
	
	}

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
    
	
	impl TryFrom<AtspiEvent> for ObjectEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"PropertyChange" => Ok(ObjectEvents::PropertyChange(PropertyChangeEvent(ev))),

				"BoundsChanged" => Ok(ObjectEvents::BoundsChanged(BoundsChangedEvent(ev))),

				"LinkSelected" => Ok(ObjectEvents::LinkSelected(LinkSelectedEvent(ev))),

				"StateChanged" => Ok(ObjectEvents::StateChanged(StateChangedEvent(ev))),

				"ChildrenChanged" => Ok(ObjectEvents::ChildrenChanged(ChildrenChangedEvent(ev))),

				"VisibleDataChanged" => Ok(ObjectEvents::VisibleDataChanged(VisibleDataChangedEvent(ev))),

				"SelectionChanged" => Ok(ObjectEvents::SelectionChanged(SelectionChangedEvent(ev))),

				"ModelChanged" => Ok(ObjectEvents::ModelChanged(ModelChangedEvent(ev))),

				"ActiveDescendantChanged" => Ok(ObjectEvents::ActiveDescendantChanged(ActiveDescendantChangedEvent(ev))),

				"Announcement" => Ok(ObjectEvents::Announcement(AnnouncementEvent(ev))),

				"AttributesChanged" => Ok(ObjectEvents::AttributesChanged(AttributesChangedEvent(ev))),

				"RowInserted" => Ok(ObjectEvents::RowInserted(RowInsertedEvent(ev))),

				"RowReordered" => Ok(ObjectEvents::RowReordered(RowReorderedEvent(ev))),

				"RowDeleted" => Ok(ObjectEvents::RowDeleted(RowDeletedEvent(ev))),

				"ColumnInserted" => Ok(ObjectEvents::ColumnInserted(ColumnInsertedEvent(ev))),

				"ColumnReordered" => Ok(ObjectEvents::ColumnReordered(ColumnReorderedEvent(ev))),

				"ColumnDeleted" => Ok(ObjectEvents::ColumnDeleted(ColumnDeletedEvent(ev))),

				"TextBoundsChanged" => Ok(ObjectEvents::TextBoundsChanged(TextBoundsChangedEvent(ev))),

				"TextSelectionChanged" => Ok(ObjectEvents::TextSelectionChanged(TextSelectionChangedEvent(ev))),

				"TextChanged" => Ok(ObjectEvents::TextChanged(TextChangedEvent(ev))),

				"TextAttributesChanged" => Ok(ObjectEvents::TextAttributesChanged(TextAttributesChangedEvent(ev))),

				"TextCaretMoved" => Ok(ObjectEvents::TextCaretMoved(TextCaretMovedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Object".into())),
			}
		}
	}
	
}
	


#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod window {
	use atspi_macros::TrySignify;
	use crate::{
        Event,
		error::AtspiError,
		events::{AtspiEvent, GenericEvent, EventInterfaces},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
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
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///          let Event::Interfaces(EventInterfaces::Window(_event)) = ev else { continue };
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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = PropertyChangeEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct PropertyChangeEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = MinimizeEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct MinimizeEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = MaximizeEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct MaximizeEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = RestoreEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RestoreEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = CloseEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct CloseEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = CreateEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct CreateEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ReparentEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ReparentEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = DesktopCreateEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DesktopCreateEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = DesktopDestroyEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DesktopDestroyEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = DestroyEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DestroyEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ActivateEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ActivateEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = DeactivateEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DeactivateEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = RaiseEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RaiseEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = LowerEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LowerEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = MoveEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct MoveEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ResizeEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ResizeEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ShadeEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ShadeEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = UUshadeEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct UUshadeEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = RestyleEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RestyleEvent(pub(crate) AtspiEvent);
	
	
    

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
    
	
	impl TryFrom<AtspiEvent> for WindowEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"PropertyChange" => Ok(WindowEvents::PropertyChange(PropertyChangeEvent(ev))),

				"Minimize" => Ok(WindowEvents::Minimize(MinimizeEvent(ev))),

				"Maximize" => Ok(WindowEvents::Maximize(MaximizeEvent(ev))),

				"Restore" => Ok(WindowEvents::Restore(RestoreEvent(ev))),

				"Close" => Ok(WindowEvents::Close(CloseEvent(ev))),

				"Create" => Ok(WindowEvents::Create(CreateEvent(ev))),

				"Reparent" => Ok(WindowEvents::Reparent(ReparentEvent(ev))),

				"DesktopCreate" => Ok(WindowEvents::DesktopCreate(DesktopCreateEvent(ev))),

				"DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(DesktopDestroyEvent(ev))),

				"Destroy" => Ok(WindowEvents::Destroy(DestroyEvent(ev))),

				"Activate" => Ok(WindowEvents::Activate(ActivateEvent(ev))),

				"Deactivate" => Ok(WindowEvents::Deactivate(DeactivateEvent(ev))),

				"Raise" => Ok(WindowEvents::Raise(RaiseEvent(ev))),

				"Lower" => Ok(WindowEvents::Lower(LowerEvent(ev))),

				"Move" => Ok(WindowEvents::Move(MoveEvent(ev))),

				"Resize" => Ok(WindowEvents::Resize(ResizeEvent(ev))),

				"Shade" => Ok(WindowEvents::Shade(ShadeEvent(ev))),

				"uUshade" => Ok(WindowEvents::UUshade(UUshadeEvent(ev))),

				"Restyle" => Ok(WindowEvents::Restyle(RestyleEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
			}
		}
	}
	
}
	


#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod mouse {
	use atspi_macros::TrySignify;
	use crate::{
        Event,
		error::AtspiError,
		events::{AtspiEvent, GenericEvent, EventInterfaces},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
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
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///          let Event::Interfaces(EventInterfaces::Mouse(_event)) = ev else { continue };
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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = AbsEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AbsEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = RelEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RelEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ButtonEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ButtonEvent(pub(crate) AtspiEvent);
	
	
    
	impl AbsEvent {
		
		#[must_use]
		pub fn x(&self) -> i32 {
			self.0.detail1()
		}
	


		#[must_use]
		pub fn y(&self) -> i32 {
			self.0.detail2()
		}
	
	}

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
    

    
	impl RelEvent {
		
		#[must_use]
		pub fn x(&self) -> i32 {
			self.0.detail1()
		}
	


		#[must_use]
		pub fn y(&self) -> i32 {
			self.0.detail2()
		}
	
	}

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
    

    
	impl ButtonEvent {
		
		#[must_use]
		pub fn mouse_x(&self) -> i32 {
			self.0.detail1()
		}
	


		#[must_use]
		pub fn mouse_y(&self) -> i32 {
			self.0.detail2()
		}
	
	}

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
    
	
	impl TryFrom<AtspiEvent> for MouseEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Abs" => Ok(MouseEvents::Abs(AbsEvent(ev))),

				"Rel" => Ok(MouseEvents::Rel(RelEvent(ev))),

				"Button" => Ok(MouseEvents::Button(ButtonEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
			}
		}
	}
	
}
	


#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod keyboard {
	use atspi_macros::TrySignify;
	use crate::{
        Event,
		error::AtspiError,
		events::{AtspiEvent, GenericEvent, EventInterfaces},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
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
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///          let Event::Interfaces(EventInterfaces::Keyboard(_event)) = ev else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum KeyboardEvents {
		Modifiers(ModifiersEvent),
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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ModifiersEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ModifiersEvent(pub(crate) AtspiEvent);
	
	
    
	impl ModifiersEvent {
		
		#[must_use]
		pub fn previous_modifiers(&self) -> i32 {
			self.0.detail1()
		}
	


		#[must_use]
		pub fn current_modifiers(&self) -> i32 {
			self.0.detail2()
		}
	
	}

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
    
	
	impl TryFrom<AtspiEvent> for KeyboardEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Modifiers" => Ok(KeyboardEvents::Modifiers(ModifiersEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
			}
		}
	}
	
}
	


#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod terminal {
	use atspi_macros::TrySignify;
	use crate::{
        Event,
		error::AtspiError,
		events::{AtspiEvent, GenericEvent, EventInterfaces},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
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
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///          let Event::Interfaces(EventInterfaces::Terminal(_event)) = ev else { continue };
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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = LineChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LineChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ColumnCountChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnCountChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = LineCountChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LineCountChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ApplicationChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ApplicationChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = CharWidthChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct CharWidthChangedEvent(pub(crate) AtspiEvent);
	
	
    

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
    
	
	impl TryFrom<AtspiEvent> for TerminalEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LineChanged" => Ok(TerminalEvents::LineChanged(LineChangedEvent(ev))),

				"ColumncountChanged" => Ok(TerminalEvents::ColumnCountChanged(ColumnCountChangedEvent(ev))),

				"LinecountChanged" => Ok(TerminalEvents::LineCountChanged(LineCountChangedEvent(ev))),

				"ApplicationChanged" => Ok(TerminalEvents::ApplicationChanged(ApplicationChangedEvent(ev))),

				"CharwidthChanged" => Ok(TerminalEvents::CharWidthChanged(CharWidthChangedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
			}
		}
	}
	
}
	


#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod document {
	use atspi_macros::TrySignify;
	use crate::{
        Event,
		error::AtspiError,
		events::{AtspiEvent, GenericEvent, EventInterfaces},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
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
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///          let Event::Interfaces(EventInterfaces::Document(_event)) = ev else { continue };
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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = LoadCompleteEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LoadCompleteEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ReloadEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ReloadEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = LoadStoppedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LoadStoppedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = ContentChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ContentChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = AttributesChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AttributesChangedEvent(pub(crate) AtspiEvent);
	

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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = PageChangedEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct PageChangedEvent(pub(crate) AtspiEvent);
	
	
    

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
    
	
	impl TryFrom<AtspiEvent> for DocumentEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LoadComplete" => Ok(DocumentEvents::LoadComplete(LoadCompleteEvent(ev))),

				"Reload" => Ok(DocumentEvents::Reload(ReloadEvent(ev))),

				"LoadStopped" => Ok(DocumentEvents::LoadStopped(LoadStoppedEvent(ev))),

				"ContentChanged" => Ok(DocumentEvents::ContentChanged(ContentChangedEvent(ev))),

				"AttributesChanged" => Ok(DocumentEvents::AttributesChanged(AttributesChangedEvent(ev))),

				"PageChanged" => Ok(DocumentEvents::PageChanged(PageChangedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
			}
		}
	}
	
}
	


#[allow(clippy::module_name_repetitions)]
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
pub mod focus {
	use atspi_macros::TrySignify;
	use crate::{
        Event,
		error::AtspiError,
		events::{AtspiEvent, GenericEvent, EventInterfaces},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
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
    /// # use std::time::Duration;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///          let Event::Interfaces(EventInterfaces::Focus(_event)) = ev else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Clone, Debug)]
	pub enum FocusEvents {
		Focus(FocusEvent),
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
    ///     let atspi = atspi::Connection::open().await.unwrap();
    ///     let events = atspi.event_stream();
    /// # let events = tokio_stream::StreamExt::timeout(events, Duration::from_secs(1));
    ///     tokio::pin!(events);
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    /// #       let Ok(ev) = ev else { break };
    ///         let Ok(event)  = FocusEvent::try_from(ev) else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct FocusEvent(pub(crate) AtspiEvent);
	
	
    

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
    
	
	impl TryFrom<AtspiEvent> for FocusEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Focus" => Ok(FocusEvents::Focus(FocusEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
			}
		}
	}
	
}
	use crate::Event;
use crate::events::{AddAccessibleEvent,RemoveAccessibleEvent,CacheEvents};
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
use crate::events::{EventListenerRegisteredEvent,EventListenerDeregisteredEvent,EventListenerEvents};
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
use crate::events::{AvailableEvent};
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
