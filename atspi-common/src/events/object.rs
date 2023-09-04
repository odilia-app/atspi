use std::hash::Hash;

use crate::{
	error::AtspiError,
	events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
	Event, State,
};
use zvariant::{ObjectPath, OwnedValue, Value};

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

impl_from_interface_event_enum_for_event!(ObjectEvents, Event::Object);
impl_try_from_event_for_user_facing_event_type!(ObjectEvents, Event::Object);

event_wrapper_test_cases!(ObjectEvents, PropertyChangeEvent);

impl HasMatchRule for ObjectEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object'";
}

/// The `org.a11y.atspi.Event.Object:PropertyChange` event.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PropertyChangeEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	pub property: String,
	pub value: Property,
}

impl Hash for PropertyChangeEvent {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.item.hash(state);
		self.property.hash(state);
	}
}

// Do not derive Eq if not all fields implement Eq
impl Eq for PropertyChangeEvent {}

// Looks like a false positive Clippy lint
#[allow(clippy::derivable_impls)]
impl Default for PropertyChangeEvent {
	fn default() -> Self {
		Self {
			item: Accessible::default(),
			property: String::default(),
			value: Property::default(),
		}
	}
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Property {
	Name(String),
	Description(String),
	Role(crate::Role),
	Parent(Accessible),
	TableCaption(String),
	TableColumnDescription(String),
	TableColumnHeader(String),
	TableRowDescription(String),
	TableRowHeader(String),
	TableSummary(String),
	Other((String, OwnedValue)),
}

impl Default for Property {
	fn default() -> Self {
		Self::Other((String::default(), zvariant::Value::U64(0).into()))
	}
}

impl TryFrom<EventBodyOwned> for Property {
	type Error = AtspiError;

	fn try_from(body: EventBodyOwned) -> Result<Self, Self::Error> {
		let property = body.kind;

		match property.as_str() {
			"accessible-name" => Ok(Self::Name(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("accessible-name"))?,
			)),
			"accessible-description" => Ok(Self::Description(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("accessible-description"))?,
			)),
			"accessible-role" => Ok(Self::Role({
				let role_int: u32 = body
					.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("accessible-role"))?;
				let role: crate::Role = crate::Role::try_from(role_int)
					.map_err(|_| AtspiError::ParseError("accessible-role"))?;
				role
			})),
			"accessible-parent" => Ok(Self::Parent(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("accessible-parent"))?,
			)),
			"accessible-table-caption" => Ok(Self::TableCaption(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("accessible-table-caption"))?,
			)),
			"table-column-description" => Ok(Self::TableColumnDescription(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("table-column-description"))?,
			)),
			"table-column-header" => Ok(Self::TableColumnHeader(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("table-column-header"))?,
			)),
			"table-row-description" => Ok(Self::TableRowDescription(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("table-row-description"))?,
			)),
			"table-row-header" => Ok(Self::TableRowHeader(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("table-row-header"))?,
			)),
			"table-summary" => Ok(Self::TableSummary(
				body.any_data
					.try_into()
					.map_err(|_| AtspiError::ParseError("table-summary"))?,
			)),
			_ => Ok(Self::Other((property, body.any_data))),
		}
	}
}

impl From<Property> for OwnedValue {
	fn from(property: Property) -> Self {
		match property {
			Property::Name(name) => Value::from(name).into(),
			Property::Description(description) => Value::from(description).into(),
			Property::Role(role) => Value::from(role as u32).into(),
			Property::Parent(parent) => Value::from(parent).into(),
			Property::TableCaption(table_caption) => Value::from(table_caption).into(),
			Property::TableColumnDescription(table_column_description) => {
				Value::from(table_column_description).into()
			}
			Property::TableColumnHeader(table_column_header) => {
				Value::from(table_column_header).into()
			}
			Property::TableRowDescription(table_row_description) => {
				Value::from(table_row_description).into()
			}
			Property::TableRowHeader(table_row_header) => Value::from(table_row_header).into(),
			Property::TableSummary(table_summary) => Value::from(table_summary).into(),
			Property::Other((_, value)) => value,
		}
	}
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct BoundsChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LinkSelectedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// A state of an object has been modified.
/// A [`State`] can be added or removed from any [`Accessible`].
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct StateChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	/// The state to be enabled/disabled.
	pub state: State,
	/// Enabled or disabled the state.
	///
	/// 1 == enabled
	///
	/// 0 == disabled
	pub enabled: i32,
}

/// A child of an [`Accessible`] has been added or removed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ChildrenChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	/// Operation, which may be one of:
	///
	/// * "insert/system"
	/// * "insert"
	/// * "delete/system"
	/// * "delete"
	///
	/// The operation is the same whether it contains the "/system" suffix or not.
	/// TODO: This should be an enum.
	pub operation: String,
	/// Index to remove from/add to.
	pub index_in_parent: i32,
	/// A reference to the new child.
	pub child: Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct VisibleDataChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct SelectionChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ModelChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ActiveDescendantChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	pub child: Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AnnouncementEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	/// Text of the announcement.
	pub text: String,
	/// Politeness level.
	pub live: crate::Live,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AttributesChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// A row has been added to a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RowInsertedEvent {
	/// The table which has had a row inserted.
	pub item: crate::events::Accessible,
}

/// A row has been moved within a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RowReorderedEvent {
	/// The table which has had a row re-ordered.
	pub item: crate::events::Accessible,
}

/// A row has been deleted from a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RowDeletedEvent {
	/// The table which has had a row removed.
	pub item: crate::events::Accessible,
}

/// A column has been added to a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnInsertedEvent {
	/// The table which has had a column inserted.
	pub item: crate::events::Accessible,
}

/// A column has been re-ordered within a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnReorderedEvent {
	/// The table which has had a column re-ordered.
	pub item: crate::events::Accessible,
}

/// A column has been removed from a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ColumnDeletedEvent {
	/// The table which has had a column removed.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextBoundsChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextSelectionChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// Text has changed within an [`Accessible`].
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
	/// Operation, which may be one of:
	///
	/// * "insert/system"
	/// * "insert"
	/// * "delete/system"
	/// * "delete"
	///
	/// The operation is the same whether it contains the "/system" suffix or not.
	/// TODO: This should be an enum.
	pub operation: String,
	/// starting index of the insertion/deletion
	pub start_pos: i32,
	/// length of the insertion/deletion
	pub length: i32,
	/// the text being inserted/deleted
	pub text: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextAttributesChangedEvent {
	/// The [`Accessible`] which the event applies to.
	pub item: crate::events::Accessible,
}

/// The caret of the user also known as a cursor (not to be confused with mouse pointer) has changed position.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct TextCaretMovedEvent {
	/// The object on which the caret has been moved on.
	pub item: crate::events::Accessible,
	/// New position of the caret.
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
		let property = body.kind.clone();
		let value: Property = body.try_into()?;
		Ok(Self { item, property, value })
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

impl GenericEvent<'_> for StateChangedEvent {
	const DBUS_MEMBER: &'static str = "StateChanged";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self { item, state: body.kind.into(), enabled: body.detail1 })
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

impl GenericEvent<'_> for AnnouncementEvent {
	const DBUS_MEMBER: &'static str = "Announcement";
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

	type Body = EventBodyOwned;

	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
		Ok(Self {
			item,
			text: body.any_data.try_into().map_err(|_| AtspiError::Conversion("text"))?,
			live: body.detail1.try_into()?,
		})
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
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'";
	const REGISTRY_EVENT_STRING: &'static str = "Object:";

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

impl_from_user_facing_event_for_interface_event_enum!(
	PropertyChangeEvent,
	ObjectEvents,
	ObjectEvents::PropertyChange
);
impl_from_user_facing_type_for_event_enum!(PropertyChangeEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	PropertyChangeEvent,
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
			any_data: event.value.into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	BoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::BoundsChanged
);
impl_from_user_facing_type_for_event_enum!(BoundsChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	BoundsChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	LinkSelectedEvent,
	ObjectEvents,
	ObjectEvents::LinkSelected
);
impl_from_user_facing_type_for_event_enum!(LinkSelectedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	LinkSelectedEvent,
	ObjectEvents::LinkSelected,
	Event::Object
);
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

impl_from_user_facing_event_for_interface_event_enum!(
	StateChangedEvent,
	ObjectEvents,
	ObjectEvents::StateChanged
);
impl_from_user_facing_type_for_event_enum!(StateChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	StateChangedEvent,
	ObjectEvents::StateChanged,
	Event::Object
);
event_test_cases!(StateChangedEvent);
impl_to_dbus_message!(StateChangedEvent);
impl_from_dbus_message!(StateChangedEvent);
impl From<StateChangedEvent> for EventBodyOwned {
	fn from(event: StateChangedEvent) -> Self {
		EventBodyOwned {
			properties: std::collections::HashMap::new(),
			kind: event.state.into(),
			detail1: event.enabled,
			detail2: i32::default(),
			any_data: zvariant::Value::U8(0).into(),
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	ChildrenChangedEvent,
	ObjectEvents,
	ObjectEvents::ChildrenChanged
);
impl_from_user_facing_type_for_event_enum!(ChildrenChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	ChildrenChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	VisibleDataChangedEvent,
	ObjectEvents,
	ObjectEvents::VisibleDataChanged
);
impl_from_user_facing_type_for_event_enum!(VisibleDataChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	VisibleDataChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	SelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::SelectionChanged
);
impl_from_user_facing_type_for_event_enum!(SelectionChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	SelectionChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	ModelChangedEvent,
	ObjectEvents,
	ObjectEvents::ModelChanged
);
impl_from_user_facing_type_for_event_enum!(ModelChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	ModelChangedEvent,
	ObjectEvents::ModelChanged,
	Event::Object
);
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

impl_from_user_facing_event_for_interface_event_enum!(
	ActiveDescendantChangedEvent,
	ObjectEvents,
	ObjectEvents::ActiveDescendantChanged
);
impl_from_user_facing_type_for_event_enum!(ActiveDescendantChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	ActiveDescendantChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	AnnouncementEvent,
	ObjectEvents,
	ObjectEvents::Announcement
);
impl_from_user_facing_type_for_event_enum!(AnnouncementEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	AnnouncementEvent,
	ObjectEvents::Announcement,
	Event::Object
);
event_test_cases!(AnnouncementEvent);
impl_to_dbus_message!(AnnouncementEvent);
impl_from_dbus_message!(AnnouncementEvent);
impl From<AnnouncementEvent> for EventBodyOwned {
	fn from(event: AnnouncementEvent) -> Self {
		EventBodyOwned {
			detail1: event.live as i32,
			any_data: zvariant::Value::from(event.text).into(),
			..Default::default()
		}
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	AttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::AttributesChanged
);
impl_from_user_facing_type_for_event_enum!(AttributesChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	AttributesChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	RowInsertedEvent,
	ObjectEvents,
	ObjectEvents::RowInserted
);
impl_from_user_facing_type_for_event_enum!(RowInsertedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	RowInsertedEvent,
	ObjectEvents::RowInserted,
	Event::Object
);
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

impl_from_user_facing_event_for_interface_event_enum!(
	RowReorderedEvent,
	ObjectEvents,
	ObjectEvents::RowReordered
);
impl_from_user_facing_type_for_event_enum!(RowReorderedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	RowReorderedEvent,
	ObjectEvents::RowReordered,
	Event::Object
);
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

impl_from_user_facing_event_for_interface_event_enum!(
	RowDeletedEvent,
	ObjectEvents,
	ObjectEvents::RowDeleted
);
impl_from_user_facing_type_for_event_enum!(RowDeletedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(RowDeletedEvent, ObjectEvents::RowDeleted, Event::Object);
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

impl_from_user_facing_event_for_interface_event_enum!(
	ColumnInsertedEvent,
	ObjectEvents,
	ObjectEvents::ColumnInserted
);
impl_from_user_facing_type_for_event_enum!(ColumnInsertedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	ColumnInsertedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	ColumnReorderedEvent,
	ObjectEvents,
	ObjectEvents::ColumnReordered
);
impl_from_user_facing_type_for_event_enum!(ColumnReorderedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	ColumnReorderedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	ColumnDeletedEvent,
	ObjectEvents,
	ObjectEvents::ColumnDeleted
);
impl_from_user_facing_type_for_event_enum!(ColumnDeletedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	ColumnDeletedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	TextBoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::TextBoundsChanged
);
impl_from_user_facing_type_for_event_enum!(TextBoundsChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	TextBoundsChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	TextSelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::TextSelectionChanged
);
impl_from_user_facing_type_for_event_enum!(TextSelectionChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	TextSelectionChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	TextChangedEvent,
	ObjectEvents,
	ObjectEvents::TextChanged
);
impl_from_user_facing_type_for_event_enum!(TextChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	TextChangedEvent,
	ObjectEvents::TextChanged,
	Event::Object
);
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

impl_from_user_facing_event_for_interface_event_enum!(
	TextAttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::TextAttributesChanged
);
impl_from_user_facing_type_for_event_enum!(TextAttributesChangedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	TextAttributesChangedEvent,
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

impl_from_user_facing_event_for_interface_event_enum!(
	TextCaretMovedEvent,
	ObjectEvents,
	ObjectEvents::TextCaretMoved
);
impl_from_user_facing_type_for_event_enum!(TextCaretMovedEvent, Event::Object);
impl_try_from_event_for_user_facing_type!(
	TextCaretMovedEvent,
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
