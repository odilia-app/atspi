use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event,
};
use zbus_names::UniqueName;
use zvariant::ObjectPath;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
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
impl_event_conversions!(ObjectEvents, Event::Object);
event_wrapper_test_cases!(ObjectEvents, PropertyChangeEvent);

impl HasMatchRule for ObjectEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object'";
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PropertyChangeEvent {
	pub item: crate::events::Accessible,
	pub property: String,
	pub value: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct BoundsChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LinkSelectedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct StateChangedEvent {
	pub item: crate::events::Accessible,
	pub state: String,
	pub enabled: i32,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ChildrenChangedEvent {
	pub item: crate::events::Accessible,
	pub operation: String,
	pub index_in_parent: i32,
	pub child: Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct VisibleDataChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct SelectionChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ModelChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ActiveDescendantChangedEvent {
	pub item: crate::events::Accessible,
	pub child: Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AnnouncementEvent {
	pub item: crate::events::Accessible,
	pub text: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AttributesChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RowInsertedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RowReorderedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RowDeletedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnInsertedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnReorderedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnDeletedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextBoundsChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextSelectionChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextChangedEvent {
	pub item: crate::events::Accessible,
	pub operation: String,
	pub start_pos: i32,
	pub length: i32,
	pub text: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextAttributesChangedEvent {
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextCaretMovedEvent {
	pub item: crate::events::Accessible,
	pub position: i32,
}

impl GenericEvent<'_> for PropertyChangeEvent {
	const DBUS_MEMBER: &'static str = "PropertyChange";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, property: body.kind, value: body.any_data.try_into()? })
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
	 if let Event::Object(ObjectEvents::PropertyChange(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for BoundsChangedEvent {
	const DBUS_MEMBER: &'static str = "BoundsChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='BoundsChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for BoundsChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::BoundsChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for LinkSelectedEvent {
	const DBUS_MEMBER: &'static str = "LinkSelected";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='LinkSelected'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for LinkSelectedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::LinkSelected(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for StateChangedEvent {
	const DBUS_MEMBER: &'static str = "StateChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, state: body.kind, enabled: body.detail1 })
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
impl TryFrom<Event> for StateChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::StateChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ChildrenChangedEvent {
	const DBUS_MEMBER: &'static str = "ChildrenChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='ChildrenChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self {
			item,
			operation: body.kind,
			index_in_parent: body.detail1,
			child: body.any_data.try_into()?,
		})
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
impl TryFrom<Event> for ChildrenChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::ChildrenChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for VisibleDataChangedEvent {
	const DBUS_MEMBER: &'static str = "VisibleDataChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='VisibleDataChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for VisibleDataChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::VisibleDataChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for SelectionChangedEvent {
	const DBUS_MEMBER: &'static str = "SelectionChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='SelectionChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for SelectionChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::SelectionChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ModelChangedEvent {
	const DBUS_MEMBER: &'static str = "ModelChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='ModelChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for ModelChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::ModelChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ActiveDescendantChangedEvent {
	const DBUS_MEMBER: &'static str = "ActiveDescendantChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='ActiveDescendantChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, child: body.any_data.try_into()? })
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
impl TryFrom<Event> for ActiveDescendantChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::ActiveDescendantChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for AnnouncementEvent {
	const DBUS_MEMBER: &'static str = "Announcement";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, text: body.kind })
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
impl TryFrom<Event> for AnnouncementEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::Announcement(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for AttributesChangedEvent {
	const DBUS_MEMBER: &'static str = "AttributesChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
	 if let Event::Object(ObjectEvents::AttributesChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for RowInsertedEvent {
	const DBUS_MEMBER: &'static str = "RowInserted";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='RowInserted'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for RowInsertedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::RowInserted(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for RowReorderedEvent {
	const DBUS_MEMBER: &'static str = "RowReordered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='RowReordered'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for RowReorderedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::RowReordered(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for RowDeletedEvent {
	const DBUS_MEMBER: &'static str = "RowDeleted";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='RowDeleted'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for RowDeletedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::RowDeleted(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ColumnInsertedEvent {
	const DBUS_MEMBER: &'static str = "ColumnInserted";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnInserted'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for ColumnInsertedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::ColumnInserted(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ColumnReorderedEvent {
	const DBUS_MEMBER: &'static str = "ColumnReordered";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnReordered'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for ColumnReorderedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::ColumnReordered(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for ColumnDeletedEvent {
	const DBUS_MEMBER: &'static str = "ColumnDeleted";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnDeleted'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for ColumnDeletedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::ColumnDeleted(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for TextBoundsChangedEvent {
	const DBUS_MEMBER: &'static str = "TextBoundsChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='TextBoundsChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for TextBoundsChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::TextBoundsChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for TextSelectionChangedEvent {
	const DBUS_MEMBER: &'static str = "TextSelectionChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='TextSelectionChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for TextSelectionChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::TextSelectionChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for TextChangedEvent {
	const DBUS_MEMBER: &'static str = "TextChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='TextChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self {
			item,
			operation: body.kind,
			start_pos: body.detail1,
			length: body.detail2,
			text: body.any_data.try_into()?,
		})
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
impl TryFrom<Event> for TextChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::TextChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for TextAttributesChangedEvent {
	const DBUS_MEMBER: &'static str = "TextAttributesChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='TextAttributesChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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
impl TryFrom<Event> for TextAttributesChangedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::TextAttributesChanged(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

impl GenericEvent<'_> for TextCaretMovedEvent {
	const DBUS_MEMBER: &'static str = "TextCaretMoved";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, position: body.detail1 })
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
impl TryFrom<Event> for TextCaretMovedEvent {
type Error = AtspiError;
fn try_from(event: Event) -> Result<Self, Self::Error> {
	 if let Event::Object(ObjectEvents::TextCaretMoved(inner_event)) = event {
			Ok(inner_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}*/

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for ObjectEvents {
	type Error = AtspiError;
	fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
		let member = ev
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			"PropertyChange" => Ok(ObjectEvents::PropertyChange(ev.try_into()?)),
			"BoundsChanged" => Ok(ObjectEvents::BoundsChanged(ev.try_into()?)),
			"LinkSelected" => Ok(ObjectEvents::LinkSelected(ev.try_into()?)),
			"StateChanged" => Ok(ObjectEvents::StateChanged(ev.try_into()?)),
			"ChildrenChanged" => Ok(ObjectEvents::ChildrenChanged(ev.try_into()?)),
			"VisibleDataChanged" => Ok(ObjectEvents::VisibleDataChanged(ev.try_into()?)),
			"SelectionChanged" => Ok(ObjectEvents::SelectionChanged(ev.try_into()?)),
			"ModelChanged" => Ok(ObjectEvents::ModelChanged(ev.try_into()?)),
			"ActiveDescendantChanged" => Ok(ObjectEvents::ActiveDescendantChanged(ev.try_into()?)),
			"Announcement" => Ok(ObjectEvents::Announcement(ev.try_into()?)),
			"AttributesChanged" => Ok(ObjectEvents::AttributesChanged(ev.try_into()?)),
			"RowInserted" => Ok(ObjectEvents::RowInserted(ev.try_into()?)),
			"RowReordered" => Ok(ObjectEvents::RowReordered(ev.try_into()?)),
			"RowDeleted" => Ok(ObjectEvents::RowDeleted(ev.try_into()?)),
			"ColumnInserted" => Ok(ObjectEvents::ColumnInserted(ev.try_into()?)),
			"ColumnReordered" => Ok(ObjectEvents::ColumnReordered(ev.try_into()?)),
			"ColumnDeleted" => Ok(ObjectEvents::ColumnDeleted(ev.try_into()?)),
			"TextBoundsChanged" => Ok(ObjectEvents::TextBoundsChanged(ev.try_into()?)),
			"TextSelectionChanged" => Ok(ObjectEvents::TextSelectionChanged(ev.try_into()?)),
			"TextChanged" => Ok(ObjectEvents::TextChanged(ev.try_into()?)),
			"TextAttributesChanged" => Ok(ObjectEvents::TextAttributesChanged(ev.try_into()?)),
			"TextCaretMoved" => Ok(ObjectEvents::TextCaretMoved(ev.try_into()?)),
			_ => Err(AtspiError::MemberMatch("No matching member for Object".into())),
		}
	}
}

impl_event_conversions!(
	PropertyChangeEvent,
	ObjectEvents,
	ObjectEvents::PropertyChange,
	Event::Object
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
			any_data: zvariant::Value::from(event.value).into(),
		}
	}
}

impl_event_conversions!(
	BoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::BoundsChanged,
	Event::Object
);
event_test_cases!(BoundsChangedEvent);
impl_to_dbus_message!(BoundsChangedEvent);
impl_from_dbus_message!(BoundsChangedEvent);
impl From<BoundsChangedEvent> for EventBodyOwned {
	fn from(_event: BoundsChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(LinkSelectedEvent, ObjectEvents, ObjectEvents::LinkSelected, Event::Object);
event_test_cases!(LinkSelectedEvent);
impl_to_dbus_message!(LinkSelectedEvent);
impl_from_dbus_message!(LinkSelectedEvent);
impl From<LinkSelectedEvent> for EventBodyOwned {
	fn from(_event: LinkSelectedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(StateChangedEvent, ObjectEvents, ObjectEvents::StateChanged, Event::Object);
event_test_cases!(StateChangedEvent);
impl_to_dbus_message!(StateChangedEvent);
impl_from_dbus_message!(StateChangedEvent);
impl From<StateChangedEvent> for EventBodyOwned {
	fn from(event: StateChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.state,
			detail1: event.enabled,
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(
	ChildrenChangedEvent,
	ObjectEvents,
	ObjectEvents::ChildrenChanged,
	Event::Object
);
event_test_cases!(ChildrenChangedEvent);
impl_to_dbus_message!(ChildrenChangedEvent);
impl_from_dbus_message!(ChildrenChangedEvent);
impl From<ChildrenChangedEvent> for EventBodyOwned {
	fn from(event: ChildrenChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.operation,
			detail1: event.index_in_parent,
			detail2: i32::default(),
			any_data: zvariant::Value::from(event.child).into(),
		}
	}
}

impl_event_conversions!(
	VisibleDataChangedEvent,
	ObjectEvents,
	ObjectEvents::VisibleDataChanged,
	Event::Object
);
event_test_cases!(VisibleDataChangedEvent);
impl_to_dbus_message!(VisibleDataChangedEvent);
impl_from_dbus_message!(VisibleDataChangedEvent);
impl From<VisibleDataChangedEvent> for EventBodyOwned {
	fn from(_event: VisibleDataChangedEvent) -> Self {
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
	SelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::SelectionChanged,
	Event::Object
);
event_test_cases!(SelectionChangedEvent);
impl_to_dbus_message!(SelectionChangedEvent);
impl_from_dbus_message!(SelectionChangedEvent);
impl From<SelectionChangedEvent> for EventBodyOwned {
	fn from(_event: SelectionChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(ModelChangedEvent, ObjectEvents, ObjectEvents::ModelChanged, Event::Object);
event_test_cases!(ModelChangedEvent);
impl_to_dbus_message!(ModelChangedEvent);
impl_from_dbus_message!(ModelChangedEvent);
impl From<ModelChangedEvent> for EventBodyOwned {
	fn from(_event: ModelChangedEvent) -> Self {
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
	ActiveDescendantChangedEvent,
	ObjectEvents,
	ObjectEvents::ActiveDescendantChanged,
	Event::Object
);
event_test_cases!(ActiveDescendantChangedEvent);
impl_to_dbus_message!(ActiveDescendantChangedEvent);
impl_from_dbus_message!(ActiveDescendantChangedEvent);
impl From<ActiveDescendantChangedEvent> for EventBodyOwned {
	fn from(event: ActiveDescendantChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::from(event.child).into(),
		}
	}
}

impl_event_conversions!(AnnouncementEvent, ObjectEvents, ObjectEvents::Announcement, Event::Object);
event_test_cases!(AnnouncementEvent);
impl_to_dbus_message!(AnnouncementEvent);
impl_from_dbus_message!(AnnouncementEvent);
impl From<AnnouncementEvent> for EventBodyOwned {
	fn from(event: AnnouncementEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.text,
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(
	AttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::AttributesChanged,
	Event::Object
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

impl_event_conversions!(RowInsertedEvent, ObjectEvents, ObjectEvents::RowInserted, Event::Object);
event_test_cases!(RowInsertedEvent);
impl_to_dbus_message!(RowInsertedEvent);
impl_from_dbus_message!(RowInsertedEvent);
impl From<RowInsertedEvent> for EventBodyOwned {
	fn from(_event: RowInsertedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(RowReorderedEvent, ObjectEvents, ObjectEvents::RowReordered, Event::Object);
event_test_cases!(RowReorderedEvent);
impl_to_dbus_message!(RowReorderedEvent);
impl_from_dbus_message!(RowReorderedEvent);
impl From<RowReorderedEvent> for EventBodyOwned {
	fn from(_event: RowReorderedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(RowDeletedEvent, ObjectEvents, ObjectEvents::RowDeleted, Event::Object);
event_test_cases!(RowDeletedEvent);
impl_to_dbus_message!(RowDeletedEvent);
impl_from_dbus_message!(RowDeletedEvent);
impl From<RowDeletedEvent> for EventBodyOwned {
	fn from(_event: RowDeletedEvent) -> Self {
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
	ColumnInsertedEvent,
	ObjectEvents,
	ObjectEvents::ColumnInserted,
	Event::Object
);
event_test_cases!(ColumnInsertedEvent);
impl_to_dbus_message!(ColumnInsertedEvent);
impl_from_dbus_message!(ColumnInsertedEvent);
impl From<ColumnInsertedEvent> for EventBodyOwned {
	fn from(_event: ColumnInsertedEvent) -> Self {
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
	ColumnReorderedEvent,
	ObjectEvents,
	ObjectEvents::ColumnReordered,
	Event::Object
);
event_test_cases!(ColumnReorderedEvent);
impl_to_dbus_message!(ColumnReorderedEvent);
impl_from_dbus_message!(ColumnReorderedEvent);
impl From<ColumnReorderedEvent> for EventBodyOwned {
	fn from(_event: ColumnReorderedEvent) -> Self {
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
	ColumnDeletedEvent,
	ObjectEvents,
	ObjectEvents::ColumnDeleted,
	Event::Object
);
event_test_cases!(ColumnDeletedEvent);
impl_to_dbus_message!(ColumnDeletedEvent);
impl_from_dbus_message!(ColumnDeletedEvent);
impl From<ColumnDeletedEvent> for EventBodyOwned {
	fn from(_event: ColumnDeletedEvent) -> Self {
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
	TextBoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::TextBoundsChanged,
	Event::Object
);
event_test_cases!(TextBoundsChangedEvent);
impl_to_dbus_message!(TextBoundsChangedEvent);
impl_from_dbus_message!(TextBoundsChangedEvent);
impl From<TextBoundsChangedEvent> for EventBodyOwned {
	fn from(_event: TextBoundsChangedEvent) -> Self {
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
	TextSelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::TextSelectionChanged,
	Event::Object
);
event_test_cases!(TextSelectionChangedEvent);
impl_to_dbus_message!(TextSelectionChangedEvent);
impl_from_dbus_message!(TextSelectionChangedEvent);
impl From<TextSelectionChangedEvent> for EventBodyOwned {
	fn from(_event: TextSelectionChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: i32::default(),
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_event_conversions!(TextChangedEvent, ObjectEvents, ObjectEvents::TextChanged, Event::Object);
event_test_cases!(TextChangedEvent);
impl_to_dbus_message!(TextChangedEvent);
impl_from_dbus_message!(TextChangedEvent);
impl From<TextChangedEvent> for EventBodyOwned {
	fn from(event: TextChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.operation,
			detail1: event.start_pos,
			detail2: event.length,
			any_data: zvariant::Value::from(event.text).into(),
		}
	}
}

impl_event_conversions!(
	TextAttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::TextAttributesChanged,
	Event::Object
);
event_test_cases!(TextAttributesChangedEvent);
impl_to_dbus_message!(TextAttributesChangedEvent);
impl_from_dbus_message!(TextAttributesChangedEvent);
impl From<TextAttributesChangedEvent> for EventBodyOwned {
	fn from(_event: TextAttributesChangedEvent) -> Self {
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
	TextCaretMovedEvent,
	ObjectEvents,
	ObjectEvents::TextCaretMoved,
	Event::Object
);
event_test_cases!(TextCaretMovedEvent);
impl_to_dbus_message!(TextCaretMovedEvent);
impl_from_dbus_message!(TextCaretMovedEvent);
impl From<TextCaretMovedEvent> for EventBodyOwned {
	fn from(event: TextCaretMovedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: String::default(),
			detail1: event.position,
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl HasRegistryEventString for ObjectEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Object:";
}
