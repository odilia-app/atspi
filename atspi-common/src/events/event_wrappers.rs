#[cfg(feature = "zbus")]
use crate::events::{
	EventWrapperMessageConversion, MessageConversion, MessageConversionExt, TryFromMessage,
};
use crate::{
	error::AtspiError,
	events::{BusProperties, HasInterfaceName, HasMatchRule, HasRegistryEventString, document::*, document::AttributesChangedEvent as DocumentAttributesChangedEvent},
	EventProperties, EventTypeProperties,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;
use crate::events::{
	EventListenerEvents,
	CacheEvents,
	AvailableEvent,
	WindowEvents,
	TerminalEvents,
	MouseEvents,
	KeyboardEvents,
	FocusEvents,
};
use crate::{
	events::object::*,
	events::object::AttributesChangedEvent as ObjectAttributesChangedEvent,
};
use serde::{Serialize, Deserialize};

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
	AttributesChanged(DocumentAttributesChangedEvent),
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
			DocumentAttributesChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::AttributesChanged(
				DocumentAttributesChangedEvent::from_message_unchecked(msg)?,
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
	DocumentAttributesChangedEvent,
	DocumentEvents,
	DocumentEvents::AttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	DocumentAttributesChangedEvent,
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


/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event {
	/// See: [`DocumentEvents`].
	Document(DocumentEvents),
	/// See: [`FocusEvents`].
	Focus(FocusEvents),
	/// See: [`KeyboardEvents`].
	Keyboard(KeyboardEvents),
	/// See: [`MouseEvents`].
	Mouse(MouseEvents),
	/// See: [`ObjectEvents`].
	Object(ObjectEvents),
	/// See: [`TerminalEvents`].
	Terminal(TerminalEvents),
	/// See: [`WindowEvents`].
	Window(WindowEvents),
	/// See: [`AvailableEvent`].
	Available(AvailableEvent),
	/// See: [`CacheEvents`].
	Cache(CacheEvents),
	/// See: [`EventListenerEvents`].
	Listener(EventListenerEvents),
}

impl EventTypeProperties for Event {
  fn member(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.member(),
      Self::Focus(inner) => inner.member(),
      Self::Keyboard(inner) => inner.member(),
      Self::Mouse(inner) => inner.member(),
      Self::Object(inner) => inner.member(),
      Self::Terminal(inner) => inner.member(),
      Self::Window(inner) => inner.member(),
      Self::Available(inner) => inner.member(),
      Self::Cache(inner) => inner.member(),
      Self::Listener(inner) => inner.member(),
    }
  }
  fn interface(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.interface(),
      Self::Focus(inner) => inner.interface(),
      Self::Keyboard(inner) => inner.interface(),
      Self::Mouse(inner) => inner.interface(),
      Self::Object(inner) => inner.interface(),
      Self::Terminal(inner) => inner.interface(),
      Self::Window(inner) => inner.interface(),
      Self::Available(inner) => inner.interface(),
      Self::Cache(inner) => inner.interface(),
      Self::Listener(inner) => inner.interface(),
    }
  }
  fn match_rule(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.match_rule(),
      Self::Focus(inner) => inner.match_rule(),
      Self::Keyboard(inner) => inner.match_rule(),
      Self::Mouse(inner) => inner.match_rule(),
      Self::Object(inner) => inner.match_rule(),
      Self::Terminal(inner) => inner.match_rule(),
      Self::Window(inner) => inner.match_rule(),
      Self::Available(inner) => inner.match_rule(),
      Self::Cache(inner) => inner.match_rule(),
      Self::Listener(inner) => inner.match_rule(),
    }
  }
  fn registry_string(&self) -> &'static str {
    match self {
      Self::Document(inner) => inner.registry_string(),
      Self::Focus(inner) => inner.registry_string(),
      Self::Keyboard(inner) => inner.registry_string(),
      Self::Mouse(inner) => inner.registry_string(),
      Self::Object(inner) => inner.registry_string(),
      Self::Terminal(inner) => inner.registry_string(),
      Self::Window(inner) => inner.registry_string(),
      Self::Available(inner) => inner.registry_string(),
      Self::Cache(inner) => inner.registry_string(),
      Self::Listener(inner) => inner.registry_string(),
    }
  }
}

impl EventProperties for Event {
  fn path(&self) -> ObjectPath<'_> {
    match self {
      Self::Document(inner) => inner.path(),
      Self::Focus(inner) => inner.path(),
      Self::Keyboard(inner) => inner.path(),
      Self::Mouse(inner) => inner.path(),
      Self::Object(inner) => inner.path(),
      Self::Terminal(inner) => inner.path(),
      Self::Window(inner) => inner.path(),
      Self::Available(inner) => inner.path(),
      Self::Cache(inner) => inner.path(),
      Self::Listener(inner) => inner.path(),
    }
  }
  fn sender(&self) -> UniqueName<'_> {
    match self {
      Self::Document(inner) => inner.sender(),
      Self::Focus(inner) => inner.sender(),
      Self::Keyboard(inner) => inner.sender(),
      Self::Mouse(inner) => inner.sender(),
      Self::Object(inner) => inner.sender(),
      Self::Terminal(inner) => inner.sender(),
      Self::Window(inner) => inner.sender(),
      Self::Available(inner) => inner.sender(),
      Self::Cache(inner) => inner.sender(),
      Self::Listener(inner) => inner.sender(),
    }
  }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ObjectEvents {
	/// See: [`PropertyChangeEvent`].
	PropertyChange(PropertyChangeEvent),
	/// See: [`BoundsChangedEvent`].
	BoundsChanged(BoundsChangedEvent),
	/// See: [`LinkSelectedEvent`].
	LinkSelected(LinkSelectedEvent),
	/// See: [`StateChangedEvent`].
	StateChanged(StateChangedEvent),
	/// See: [`ChildrenChangedEvent`].
	ChildrenChanged(ChildrenChangedEvent),
	/// See: [`VisibleDataChangedEvent`].
	VisibleDataChanged(VisibleDataChangedEvent),
	/// See: [`SelectionChangedEvent`].
	SelectionChanged(SelectionChangedEvent),
	/// See: [`ModelChangedEvent`].
	ModelChanged(ModelChangedEvent),
	/// See: [`ActiveDescendantChangedEvent`].
	ActiveDescendantChanged(ActiveDescendantChangedEvent),
	/// See: [`AnnouncementEvent`].
	Announcement(AnnouncementEvent),
	/// See: [`AttributesChangedEvent`].
	AttributesChanged(ObjectAttributesChangedEvent),
	/// See: [`RowInsertedEvent`].
	RowInserted(RowInsertedEvent),
	/// See: [`RowReorderedEvent`].
	RowReordered(RowReorderedEvent),
	/// See: [`RowDeletedEvent`].
	RowDeleted(RowDeletedEvent),
	/// See: [`ColumnInsertedEvent`].
	ColumnInserted(ColumnInsertedEvent),
	/// See: [`ColumnReorderedEvent`].
	ColumnReordered(ColumnReorderedEvent),
	/// See: [`ColumnDeletedEvent`].
	ColumnDeleted(ColumnDeletedEvent),
	/// See: [`TextBoundsChangedEvent`].
	TextBoundsChanged(TextBoundsChangedEvent),
	/// See: [`TextSelectionChangedEvent`].
	TextSelectionChanged(TextSelectionChangedEvent),
	/// See: [`TextChangedEvent`].
	TextChanged(TextChangedEvent),
	/// See: [`TextAttributesChangedEvent`].
	TextAttributesChanged(TextAttributesChangedEvent),
	/// See: [`TextCaretMovedEvent`].
	TextCaretMoved(TextCaretMovedEvent),
}

impl EventTypeProperties for ObjectEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.member(),
			Self::BoundsChanged(inner) => inner.member(),
			Self::LinkSelected(inner) => inner.member(),
			Self::StateChanged(inner) => inner.member(),
			Self::ChildrenChanged(inner) => inner.member(),
			Self::VisibleDataChanged(inner) => inner.member(),
			Self::SelectionChanged(inner) => inner.member(),
			Self::ModelChanged(inner) => inner.member(),
			Self::ActiveDescendantChanged(inner) => inner.member(),
			Self::Announcement(inner) => inner.member(),
			Self::AttributesChanged(inner) => inner.member(),
			Self::RowInserted(inner) => inner.member(),
			Self::RowReordered(inner) => inner.member(),
			Self::RowDeleted(inner) => inner.member(),
			Self::ColumnInserted(inner) => inner.member(),
			Self::ColumnReordered(inner) => inner.member(),
			Self::ColumnDeleted(inner) => inner.member(),
			Self::TextBoundsChanged(inner) => inner.member(),
			Self::TextSelectionChanged(inner) => inner.member(),
			Self::TextChanged(inner) => inner.member(),
			Self::TextAttributesChanged(inner) => inner.member(),
			Self::TextCaretMoved(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.interface(),
			Self::BoundsChanged(inner) => inner.interface(),
			Self::LinkSelected(inner) => inner.interface(),
			Self::StateChanged(inner) => inner.interface(),
			Self::ChildrenChanged(inner) => inner.interface(),
			Self::VisibleDataChanged(inner) => inner.interface(),
			Self::SelectionChanged(inner) => inner.interface(),
			Self::ModelChanged(inner) => inner.interface(),
			Self::ActiveDescendantChanged(inner) => inner.interface(),
			Self::Announcement(inner) => inner.interface(),
			Self::AttributesChanged(inner) => inner.interface(),
			Self::RowInserted(inner) => inner.interface(),
			Self::RowReordered(inner) => inner.interface(),
			Self::RowDeleted(inner) => inner.interface(),
			Self::ColumnInserted(inner) => inner.interface(),
			Self::ColumnReordered(inner) => inner.interface(),
			Self::ColumnDeleted(inner) => inner.interface(),
			Self::TextBoundsChanged(inner) => inner.interface(),
			Self::TextSelectionChanged(inner) => inner.interface(),
			Self::TextChanged(inner) => inner.interface(),
			Self::TextAttributesChanged(inner) => inner.interface(),
			Self::TextCaretMoved(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.match_rule(),
			Self::BoundsChanged(inner) => inner.match_rule(),
			Self::LinkSelected(inner) => inner.match_rule(),
			Self::StateChanged(inner) => inner.match_rule(),
			Self::ChildrenChanged(inner) => inner.match_rule(),
			Self::VisibleDataChanged(inner) => inner.match_rule(),
			Self::SelectionChanged(inner) => inner.match_rule(),
			Self::ModelChanged(inner) => inner.match_rule(),
			Self::ActiveDescendantChanged(inner) => inner.match_rule(),
			Self::Announcement(inner) => inner.match_rule(),
			Self::AttributesChanged(inner) => inner.match_rule(),
			Self::RowInserted(inner) => inner.match_rule(),
			Self::RowReordered(inner) => inner.match_rule(),
			Self::RowDeleted(inner) => inner.match_rule(),
			Self::ColumnInserted(inner) => inner.match_rule(),
			Self::ColumnReordered(inner) => inner.match_rule(),
			Self::ColumnDeleted(inner) => inner.match_rule(),
			Self::TextBoundsChanged(inner) => inner.match_rule(),
			Self::TextSelectionChanged(inner) => inner.match_rule(),
			Self::TextChanged(inner) => inner.match_rule(),
			Self::TextAttributesChanged(inner) => inner.match_rule(),
			Self::TextCaretMoved(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.registry_string(),
			Self::BoundsChanged(inner) => inner.registry_string(),
			Self::LinkSelected(inner) => inner.registry_string(),
			Self::StateChanged(inner) => inner.registry_string(),
			Self::ChildrenChanged(inner) => inner.registry_string(),
			Self::VisibleDataChanged(inner) => inner.registry_string(),
			Self::SelectionChanged(inner) => inner.registry_string(),
			Self::ModelChanged(inner) => inner.registry_string(),
			Self::ActiveDescendantChanged(inner) => inner.registry_string(),
			Self::Announcement(inner) => inner.registry_string(),
			Self::AttributesChanged(inner) => inner.registry_string(),
			Self::RowInserted(inner) => inner.registry_string(),
			Self::RowReordered(inner) => inner.registry_string(),
			Self::RowDeleted(inner) => inner.registry_string(),
			Self::ColumnInserted(inner) => inner.registry_string(),
			Self::ColumnReordered(inner) => inner.registry_string(),
			Self::ColumnDeleted(inner) => inner.registry_string(),
			Self::TextBoundsChanged(inner) => inner.registry_string(),
			Self::TextSelectionChanged(inner) => inner.registry_string(),
			Self::TextChanged(inner) => inner.registry_string(),
			Self::TextAttributesChanged(inner) => inner.registry_string(),
			Self::TextCaretMoved(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for ObjectEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::PropertyChange(inner) => inner.path(),
			Self::BoundsChanged(inner) => inner.path(),
			Self::LinkSelected(inner) => inner.path(),
			Self::StateChanged(inner) => inner.path(),
			Self::ChildrenChanged(inner) => inner.path(),
			Self::VisibleDataChanged(inner) => inner.path(),
			Self::SelectionChanged(inner) => inner.path(),
			Self::ModelChanged(inner) => inner.path(),
			Self::ActiveDescendantChanged(inner) => inner.path(),
			Self::Announcement(inner) => inner.path(),
			Self::AttributesChanged(inner) => inner.path(),
			Self::RowInserted(inner) => inner.path(),
			Self::RowReordered(inner) => inner.path(),
			Self::RowDeleted(inner) => inner.path(),
			Self::ColumnInserted(inner) => inner.path(),
			Self::ColumnReordered(inner) => inner.path(),
			Self::ColumnDeleted(inner) => inner.path(),
			Self::TextBoundsChanged(inner) => inner.path(),
			Self::TextSelectionChanged(inner) => inner.path(),
			Self::TextChanged(inner) => inner.path(),
			Self::TextAttributesChanged(inner) => inner.path(),
			Self::TextCaretMoved(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::PropertyChange(inner) => inner.sender(),
			Self::BoundsChanged(inner) => inner.sender(),
			Self::LinkSelected(inner) => inner.sender(),
			Self::StateChanged(inner) => inner.sender(),
			Self::ChildrenChanged(inner) => inner.sender(),
			Self::VisibleDataChanged(inner) => inner.sender(),
			Self::SelectionChanged(inner) => inner.sender(),
			Self::ModelChanged(inner) => inner.sender(),
			Self::ActiveDescendantChanged(inner) => inner.sender(),
			Self::Announcement(inner) => inner.sender(),
			Self::AttributesChanged(inner) => inner.sender(),
			Self::RowInserted(inner) => inner.sender(),
			Self::RowReordered(inner) => inner.sender(),
			Self::RowDeleted(inner) => inner.sender(),
			Self::ColumnInserted(inner) => inner.sender(),
			Self::ColumnReordered(inner) => inner.sender(),
			Self::ColumnDeleted(inner) => inner.sender(),
			Self::TextBoundsChanged(inner) => inner.sender(),
			Self::TextSelectionChanged(inner) => inner.sender(),
			Self::TextChanged(inner) => inner.sender(),
			Self::TextAttributesChanged(inner) => inner.sender(),
			Self::TextCaretMoved(inner) => inner.sender(),
		}
	}
}

impl_from_interface_event_enum_for_event!(ObjectEvents, Event::Object);
impl_try_from_event_for_user_facing_event_type!(ObjectEvents, Event::Object);
event_wrapper_test_cases!(ObjectEvents, PropertyChangeEvent);

// TODO deez
impl_from_user_facing_event_for_interface_event_enum!(
	PropertyChangeEvent,
	ObjectEvents,
	ObjectEvents::PropertyChange
);
impl_try_from_event_for_user_facing_type!(
	PropertyChangeEvent,
	ObjectEvents::PropertyChange,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	BoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::BoundsChanged
);
impl_try_from_event_for_user_facing_type!(
	BoundsChangedEvent,
	ObjectEvents::BoundsChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	LinkSelectedEvent,
	ObjectEvents,
	ObjectEvents::LinkSelected
);
impl_try_from_event_for_user_facing_type!(
	LinkSelectedEvent,
	ObjectEvents::LinkSelected,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	StateChangedEvent,
	ObjectEvents,
	ObjectEvents::StateChanged
);
impl_try_from_event_for_user_facing_type!(
	StateChangedEvent,
	ObjectEvents::StateChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ChildrenChangedEvent,
	ObjectEvents,
	ObjectEvents::ChildrenChanged
);
impl_try_from_event_for_user_facing_type!(
	ChildrenChangedEvent,
	ObjectEvents::ChildrenChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	VisibleDataChangedEvent,
	ObjectEvents,
	ObjectEvents::VisibleDataChanged
);
impl_try_from_event_for_user_facing_type!(
	VisibleDataChangedEvent,
	ObjectEvents::VisibleDataChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	SelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::SelectionChanged
);
impl_try_from_event_for_user_facing_type!(
	SelectionChangedEvent,
	ObjectEvents::SelectionChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ModelChangedEvent,
	ObjectEvents,
	ObjectEvents::ModelChanged
);
impl_try_from_event_for_user_facing_type!(
	ModelChangedEvent,
	ObjectEvents::ModelChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ActiveDescendantChangedEvent,
	ObjectEvents,
	ObjectEvents::ActiveDescendantChanged
);
impl_try_from_event_for_user_facing_type!(
	ActiveDescendantChangedEvent,
	ObjectEvents::ActiveDescendantChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	AnnouncementEvent,
	ObjectEvents,
	ObjectEvents::Announcement
);
impl_try_from_event_for_user_facing_type!(
	AnnouncementEvent,
	ObjectEvents::Announcement,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ObjectAttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::AttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	ObjectAttributesChangedEvent,
	ObjectEvents::AttributesChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowInsertedEvent,
	ObjectEvents,
	ObjectEvents::RowInserted
);
impl_try_from_event_for_user_facing_type!(
	RowInsertedEvent,
	ObjectEvents::RowInserted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowReorderedEvent,
	ObjectEvents,
	ObjectEvents::RowReordered
);
impl_try_from_event_for_user_facing_type!(
	RowReorderedEvent,
	ObjectEvents::RowReordered,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowDeletedEvent,
	ObjectEvents,
	ObjectEvents::RowDeleted
);
impl_try_from_event_for_user_facing_type!(RowDeletedEvent, ObjectEvents::RowDeleted, Event::Object);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnInsertedEvent,
	ObjectEvents,
	ObjectEvents::ColumnInserted
);
impl_try_from_event_for_user_facing_type!(
	ColumnInsertedEvent,
	ObjectEvents::ColumnInserted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnReorderedEvent,
	ObjectEvents,
	ObjectEvents::ColumnReordered
);
impl_try_from_event_for_user_facing_type!(
	ColumnReorderedEvent,
	ObjectEvents::ColumnReordered,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnDeletedEvent,
	ObjectEvents,
	ObjectEvents::ColumnDeleted
);
impl_try_from_event_for_user_facing_type!(
	ColumnDeletedEvent,
	ObjectEvents::ColumnDeleted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextBoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::TextBoundsChanged
);
impl_try_from_event_for_user_facing_type!(
	TextBoundsChangedEvent,
	ObjectEvents::TextBoundsChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextSelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::TextSelectionChanged
);
impl_try_from_event_for_user_facing_type!(
	TextSelectionChangedEvent,
	ObjectEvents::TextSelectionChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextChangedEvent,
	ObjectEvents,
	ObjectEvents::TextChanged
);
impl_try_from_event_for_user_facing_type!(
	TextChangedEvent,
	ObjectEvents::TextChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextAttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::TextAttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	TextAttributesChangedEvent,
	ObjectEvents::TextAttributesChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextCaretMovedEvent,
	ObjectEvents,
	ObjectEvents::TextCaretMoved
);
impl_try_from_event_for_user_facing_type!(
	TextCaretMovedEvent,
	ObjectEvents::TextCaretMoved,
	Event::Object
);

impl HasMatchRule for ObjectEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object'";
}

impl HasInterfaceName for ObjectEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
}

impl HasRegistryEventString for ObjectEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Object:";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for ObjectEvents {
  fn try_from_message_interface_checked(msg: &zbus::Message) -> Result<Self, AtspiError> {
    let header = msg.header();
    let member = header.member().ok_or(AtspiError::MissingMember)?;
    match member.as_str() {
      PropertyChangeEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::PropertyChange(PropertyChangeEvent::from_message_unchecked(msg)?))
      }
      BoundsChangedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::BoundsChanged(BoundsChangedEvent::from_message_unchecked(msg)?))
      }
      LinkSelectedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::LinkSelected(LinkSelectedEvent::from_message_unchecked(msg)?))
      }
      StateChangedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::StateChanged(StateChangedEvent::from_message_unchecked(msg)?))
      }
      ChildrenChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::ChildrenChanged(
        ChildrenChangedEvent::from_message_unchecked(msg)?,
      )),
      VisibleDataChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::VisibleDataChanged(
        VisibleDataChangedEvent::from_message_unchecked(msg)?,
      )),
      SelectionChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::SelectionChanged(
        SelectionChangedEvent::from_message_unchecked(msg)?,
      )),
      ModelChangedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::ModelChanged(ModelChangedEvent::from_message_unchecked(msg)?))
      }
      ActiveDescendantChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::ActiveDescendantChanged(
        ActiveDescendantChangedEvent::from_message_unchecked(msg)?,
      )),
      AnnouncementEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::Announcement(AnnouncementEvent::from_message_unchecked(msg)?))
      }
      ObjectAttributesChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::AttributesChanged(
        ObjectAttributesChangedEvent::from_message_unchecked(msg)?,
      )),
      RowInsertedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::RowInserted(RowInsertedEvent::from_message_unchecked(msg)?))
      }
      RowReorderedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::RowReordered(RowReorderedEvent::from_message_unchecked(msg)?))
      }
      RowDeletedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::RowDeleted(RowDeletedEvent::from_message_unchecked(msg)?))
      }
      ColumnInsertedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::ColumnInserted(ColumnInsertedEvent::from_message_unchecked(msg)?))
      }
      ColumnReorderedEvent::DBUS_MEMBER => Ok(ObjectEvents::ColumnReordered(
        ColumnReorderedEvent::from_message_unchecked(msg)?,
      )),
      ColumnDeletedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::ColumnDeleted(ColumnDeletedEvent::from_message_unchecked(msg)?))
      }
      TextBoundsChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextBoundsChanged(
        TextBoundsChangedEvent::from_message_unchecked(msg)?,
      )),
      TextSelectionChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextSelectionChanged(
        TextSelectionChangedEvent::from_message_unchecked(msg)?,
      )),
      TextChangedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::TextChanged(TextChangedEvent::from_message_unchecked(msg)?))
      }
      TextAttributesChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextAttributesChanged(
        TextAttributesChangedEvent::from_message_unchecked(msg)?,
      )),
      TextCaretMovedEvent::DBUS_MEMBER => {
        Ok(ObjectEvents::TextCaretMoved(TextCaretMovedEvent::from_message_unchecked(msg)?))
      }
      _ => Err(AtspiError::MemberMatch(format!(
        "No matching member {member} for interface {}",
        Self::DBUS_INTERFACE,
      ))),
    }
  }
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for ObjectEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

