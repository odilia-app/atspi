
use crate::AtspiError;

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod object {
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, EventInterfaces, GenericEvent, HasMatchRule, HasMatchRules},
		signify::Signified,
		Event,
	};
	use atspi_macros::TrySignify;
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRules for ObjectEvents {
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {
			Ok(vec![
				<PropertyChangeEvent as HasMatchRule>::match_rule()?,
				<BoundsChangedEvent as HasMatchRule>::match_rule()?,
				<LinkSelectedEvent as HasMatchRule>::match_rule()?,
				<StateChangedEvent as HasMatchRule>::match_rule()?,
				<ChildrenChangedEvent as HasMatchRule>::match_rule()?,
				<VisibleDataChangedEvent as HasMatchRule>::match_rule()?,
				<SelectionChangedEvent as HasMatchRule>::match_rule()?,
				<ModelChangedEvent as HasMatchRule>::match_rule()?,
				<ActiveDescendantChangedEvent as HasMatchRule>::match_rule()?,
				<AnnouncementEvent as HasMatchRule>::match_rule()?,
				<AttributesChangedEvent as HasMatchRule>::match_rule()?,
				<RowInsertedEvent as HasMatchRule>::match_rule()?,
				<RowReorderedEvent as HasMatchRule>::match_rule()?,
				<RowDeletedEvent as HasMatchRule>::match_rule()?,
				<ColumnInsertedEvent as HasMatchRule>::match_rule()?,
				<ColumnReorderedEvent as HasMatchRule>::match_rule()?,
				<ColumnDeletedEvent as HasMatchRule>::match_rule()?,
				<TextBoundsChangedEvent as HasMatchRule>::match_rule()?,
				<TextSelectionChangedEvent as HasMatchRule>::match_rule()?,
				<TextChangedEvent as HasMatchRule>::match_rule()?,
				<TextAttributesChangedEvent as HasMatchRule>::match_rule()?,
				<TextCaretMovedEvent as HasMatchRule>::match_rule()?,
			])
		}
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl StateChangedEvent {
		#[must_use]
		pub fn enabled(&self) -> i32 {
			self.0.detail1()
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

	impl ActiveDescendantChangedEvent {
		#[must_use]
		pub fn child(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
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

	impl TextChangedEvent {
		#[must_use]
		pub fn start_pos(&self) -> i32 {
			self.0.detail1()
		}

		#[must_use]
		pub fn length(&self) -> i32 {
			self.0.detail2()
		}

		#[must_use]
		pub fn text(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
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

	impl TextCaretMovedEvent {
		#[must_use]
		pub fn position(&self) -> i32 {
			self.0.detail1()
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
				"VisibleDataChanged" => {
					Ok(ObjectEvents::VisibleDataChanged(VisibleDataChangedEvent(ev)))
				}
				"SelectionChanged" => Ok(ObjectEvents::SelectionChanged(SelectionChangedEvent(ev))),
				"ModelChanged" => Ok(ObjectEvents::ModelChanged(ModelChangedEvent(ev))),
				"ActiveDescendantChanged" => {
					Ok(ObjectEvents::ActiveDescendantChanged(ActiveDescendantChangedEvent(ev)))
				}
				"Announcement" => Ok(ObjectEvents::Announcement(AnnouncementEvent(ev))),
				"AttributesChanged" => {
					Ok(ObjectEvents::AttributesChanged(AttributesChangedEvent(ev)))
				}
				"RowInserted" => Ok(ObjectEvents::RowInserted(RowInsertedEvent(ev))),
				"RowReordered" => Ok(ObjectEvents::RowReordered(RowReorderedEvent(ev))),
				"RowDeleted" => Ok(ObjectEvents::RowDeleted(RowDeletedEvent(ev))),
				"ColumnInserted" => Ok(ObjectEvents::ColumnInserted(ColumnInsertedEvent(ev))),
				"ColumnReordered" => Ok(ObjectEvents::ColumnReordered(ColumnReorderedEvent(ev))),
				"ColumnDeleted" => Ok(ObjectEvents::ColumnDeleted(ColumnDeletedEvent(ev))),
				"TextBoundsChanged" => {
					Ok(ObjectEvents::TextBoundsChanged(TextBoundsChangedEvent(ev)))
				}
				"TextSelectionChanged" => {
					Ok(ObjectEvents::TextSelectionChanged(TextSelectionChangedEvent(ev)))
				}
				"TextChanged" => Ok(ObjectEvents::TextChanged(TextChangedEvent(ev))),
				"TextAttributesChanged" => {
					Ok(ObjectEvents::TextAttributesChanged(TextAttributesChangedEvent(ev)))
				}
				"TextCaretMoved" => Ok(ObjectEvents::TextCaretMoved(TextCaretMovedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Object".into())),
			}
		}
	}

	impl HasMatchRule for PropertyChangeEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "PropertyChange";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for BoundsChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "BoundsChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for LinkSelectedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "LinkSelected";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for StateChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "StateChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ChildrenChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "ChildrenChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for VisibleDataChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "VisibleDataChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for SelectionChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "SelectionChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ModelChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "ModelChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ActiveDescendantChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "ActiveDescendantChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for AnnouncementEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "Announcement";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for AttributesChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "AttributesChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for RowInsertedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "RowInserted";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for RowReorderedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "RowReordered";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for RowDeletedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "RowDeleted";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ColumnInsertedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "ColumnInserted";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ColumnReorderedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "ColumnReordered";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ColumnDeletedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "ColumnDeleted";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for TextBoundsChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "TextBoundsChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for TextSelectionChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "TextSelectionChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for TextChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "TextChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for TextAttributesChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "TextAttributesChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for TextCaretMovedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MEMBER: &'static str = "TextCaretMoved";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod window {
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, EventInterfaces, GenericEvent, HasMatchRule, HasMatchRules},
		signify::Signified,
		Event,
	};
	use atspi_macros::TrySignify;
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRules for WindowEvents {
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {
			Ok(vec![
				<PropertyChangeEvent as HasMatchRule>::match_rule()?,
				<MinimizeEvent as HasMatchRule>::match_rule()?,
				<MaximizeEvent as HasMatchRule>::match_rule()?,
				<RestoreEvent as HasMatchRule>::match_rule()?,
				<CloseEvent as HasMatchRule>::match_rule()?,
				<CreateEvent as HasMatchRule>::match_rule()?,
				<ReparentEvent as HasMatchRule>::match_rule()?,
				<DesktopCreateEvent as HasMatchRule>::match_rule()?,
				<DesktopDestroyEvent as HasMatchRule>::match_rule()?,
				<DestroyEvent as HasMatchRule>::match_rule()?,
				<ActivateEvent as HasMatchRule>::match_rule()?,
				<DeactivateEvent as HasMatchRule>::match_rule()?,
				<RaiseEvent as HasMatchRule>::match_rule()?,
				<LowerEvent as HasMatchRule>::match_rule()?,
				<MoveEvent as HasMatchRule>::match_rule()?,
				<ResizeEvent as HasMatchRule>::match_rule()?,
				<ShadeEvent as HasMatchRule>::match_rule()?,
				<UUshadeEvent as HasMatchRule>::match_rule()?,
				<RestyleEvent as HasMatchRule>::match_rule()?,
			])
		}
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRule for PropertyChangeEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "PropertyChange";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for MinimizeEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Minimize";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for MaximizeEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Maximize";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for RestoreEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Restore";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for CloseEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Close";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for CreateEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Create";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ReparentEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Reparent";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for DesktopCreateEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "DesktopCreate";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for DesktopDestroyEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "DesktopDestroy";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for DestroyEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Destroy";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ActivateEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Activate";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for DeactivateEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Deactivate";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for RaiseEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Raise";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for LowerEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Lower";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for MoveEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Move";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ResizeEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Resize";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ShadeEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Shade";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for UUshadeEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "uUshade";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for RestyleEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MEMBER: &'static str = "Restyle";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod mouse {
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, EventInterfaces, GenericEvent, HasMatchRule, HasMatchRules},
		signify::Signified,
		Event,
	};
	use atspi_macros::TrySignify;
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRules for MouseEvents {
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {
			Ok(vec![
				<AbsEvent as HasMatchRule>::match_rule()?,
				<RelEvent as HasMatchRule>::match_rule()?,
				<ButtonEvent as HasMatchRule>::match_rule()?,
			])
		}
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRule for AbsEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MEMBER: &'static str = "Abs";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for RelEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MEMBER: &'static str = "Rel";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ButtonEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MEMBER: &'static str = "Button";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod keyboard {
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, EventInterfaces, GenericEvent, HasMatchRule, HasMatchRules},
		signify::Signified,
		Event,
	};
	use atspi_macros::TrySignify;
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRules for KeyboardEvents {
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {
			Ok(vec![<ModifiersEvent as HasMatchRule>::match_rule()?])
		}
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRule for ModifiersEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
		const MEMBER: &'static str = "Modifiers";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod terminal {
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, EventInterfaces, GenericEvent, HasMatchRule, HasMatchRules},
		signify::Signified,
		Event,
	};
	use atspi_macros::TrySignify;
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRules for TerminalEvents {
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {
			Ok(vec![
				<LineChangedEvent as HasMatchRule>::match_rule()?,
				<ColumnCountChangedEvent as HasMatchRule>::match_rule()?,
				<LineCountChangedEvent as HasMatchRule>::match_rule()?,
				<ApplicationChangedEvent as HasMatchRule>::match_rule()?,
				<CharWidthChangedEvent as HasMatchRule>::match_rule()?,
			])
		}
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl TryFrom<AtspiEvent> for TerminalEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LineChanged" => Ok(TerminalEvents::LineChanged(LineChangedEvent(ev))),
				"ColumncountChanged" => {
					Ok(TerminalEvents::ColumnCountChanged(ColumnCountChangedEvent(ev)))
				}
				"LinecountChanged" => {
					Ok(TerminalEvents::LineCountChanged(LineCountChangedEvent(ev)))
				}
				"ApplicationChanged" => {
					Ok(TerminalEvents::ApplicationChanged(ApplicationChangedEvent(ev)))
				}
				"CharwidthChanged" => {
					Ok(TerminalEvents::CharWidthChanged(CharWidthChangedEvent(ev)))
				}
				_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
			}
		}
	}

	impl HasMatchRule for LineChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MEMBER: &'static str = "LineChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ColumnCountChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MEMBER: &'static str = "ColumncountChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for LineCountChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MEMBER: &'static str = "LinecountChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ApplicationChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MEMBER: &'static str = "ApplicationChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for CharWidthChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MEMBER: &'static str = "CharwidthChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod document {
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, EventInterfaces, GenericEvent, HasMatchRule, HasMatchRules},
		signify::Signified,
		Event,
	};
	use atspi_macros::TrySignify;
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRules for DocumentEvents {
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {
			Ok(vec![
				<LoadCompleteEvent as HasMatchRule>::match_rule()?,
				<ReloadEvent as HasMatchRule>::match_rule()?,
				<LoadStoppedEvent as HasMatchRule>::match_rule()?,
				<ContentChangedEvent as HasMatchRule>::match_rule()?,
				<AttributesChangedEvent as HasMatchRule>::match_rule()?,
				<PageChangedEvent as HasMatchRule>::match_rule()?,
			])
		}
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl TryFrom<AtspiEvent> for DocumentEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LoadComplete" => Ok(DocumentEvents::LoadComplete(LoadCompleteEvent(ev))),
				"Reload" => Ok(DocumentEvents::Reload(ReloadEvent(ev))),
				"LoadStopped" => Ok(DocumentEvents::LoadStopped(LoadStoppedEvent(ev))),
				"ContentChanged" => Ok(DocumentEvents::ContentChanged(ContentChangedEvent(ev))),
				"AttributesChanged" => {
					Ok(DocumentEvents::AttributesChanged(AttributesChangedEvent(ev)))
				}
				"PageChanged" => Ok(DocumentEvents::PageChanged(PageChangedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
			}
		}
	}

	impl HasMatchRule for LoadCompleteEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MEMBER: &'static str = "LoadComplete";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ReloadEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MEMBER: &'static str = "Reload";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for LoadStoppedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MEMBER: &'static str = "LoadStopped";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for ContentChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MEMBER: &'static str = "ContentChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for AttributesChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MEMBER: &'static str = "AttributesChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
	impl HasMatchRule for PageChangedEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MEMBER: &'static str = "PageChanged";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod focus {
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, EventInterfaces, GenericEvent, HasMatchRule, HasMatchRules},
		signify::Signified,
		Event,
	};
	use atspi_macros::TrySignify;
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRules for FocusEvents {
		fn match_rules() -> Result<Vec<zbus::MatchRule<'static>>, AtspiError> {
			Ok(vec![<FocusEvent as HasMatchRule>::match_rule()?])
		}
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
	///     let atspi = atspi::AccessibilityBus::open().await.unwrap();
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

	impl HasMatchRule for FocusEvent {
		const INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
		const MEMBER: &'static str = "Focus";
		fn match_rule() -> Result<zbus::MatchRule<'static>, AtspiError> {
			Ok(zbus::MatchRule::builder()
				.msg_type(zbus::MessageType::Signal)
				.interface(<Self as HasMatchRule>::INTERFACE)?
				.member(<Self as HasMatchRule>::MEMBER)?
				.build())
		}
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
