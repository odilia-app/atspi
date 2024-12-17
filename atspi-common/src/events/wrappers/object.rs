use zbus_names::UniqueName;
use zvariant::ObjectPath;
use crate::{
	EventTypeProperties,
	EventProperties,
	AtspiError,
	Event,
	events::object::*,
	events::{
		traits::{TryFromMessage, BusProperties},
		MessageConversion,
		HasMatchRule,
		HasInterfaceName,
		HasRegistryEventString,
		EventWrapperMessageConversion,
	},
};
use serde::{Serialize, Deserialize};

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
	AttributesChanged(AttributesChangedEvent),
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
      AttributesChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::AttributesChanged(
        AttributesChangedEvent::from_message_unchecked(msg)?,
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

