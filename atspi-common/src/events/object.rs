//! All events which can be received by the `org.a11y.atspi.Object` interface.

#![deny(missing_docs)]

#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::object_ref::NonNullObjectRef;
#[cfg(feature = "zbus")]
use crate::EventProperties;
use crate::{
	error::AtspiError,
	events::{
		DBusInterface, DBusMatchRule, DBusMember, EventBody, EventBodyOwned, RegistryEventString,
	},
	object_ref::ObjectRefOwned,
	State,
};
use std::hash::Hash;
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};
use zvariant::{OwnedValue, Value};

const ACCESSIBLE_NAME_PROPERTY_NAME: &str = "accessible-name";
const ACCESSIBLE_DESCRIPTION_PROPERTY_NAME: &str = "accessible-description";
const ACCESSIBLE_HELP_TEXT_PROPERTY_NAME: &str = "accessible-help-text";
const ACCESSIBLE_PARENT_PROPERTY_NAME: &str = "accessible-parent";
const ACCESSIBLE_ROLE_PROPERTY_NAME: &str = "accessible-role";
const ACCESSIBLE_TABLE_CAPTION_PROPERTY_NAME: &str = "accessible-table-caption";
const ACCESSIBLE_TABLE_COLUMN_DESCRIPTION_PROPERTY_NAME: &str =
	"accessible-table-column-description";
const ACCESSIBLE_TABLE_COLUMN_HEADER_PROPERTY_NAME: &str = "accessible-table-column-header";
const ACCESSIBLE_TABLE_ROW_DESCRIPTION_PROPERTY_NAME: &str = "accessible-table-row-description";
const ACCESSIBLE_TABLE_ROW_HEADER_PROPERTY_NAME: &str = "accessible-table-row-header";
const ACCESSIBLE_TABLE_SUMMARY_PROPERTY_NAME: &str = "accessible-table-summary";

/// An event representing a property change on UI item `item` with new value `value`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PropertyChangeEvent<'a> {
	/// The [`crate::ObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// The name of the property.
	// TODO: this is not necessary since the string is encoded in the `Property` type.
	pub property: String,
	/// The value of the property.
	pub value: Property,
}

impl_event_type_properties_for_event!(PropertyChangeEvent<'_>);

impl Hash for PropertyChangeEvent<'_> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.item.hash(state);
		self.property.hash(state);
	}

	fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
	where
		Self: Sized,
	{
		for piece in data {
			piece.hash(state);
		}
	}
}

// Do not derive Eq if not all fields implement Eq
impl Eq for PropertyChangeEvent<'_> {}

/// Any accessibility related property on an [`crate::ObjectRef`].
/// This is used only in the [`PropertyChangeEvent`]; this event gets triggered if a role or accessible
/// description of an item changes.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Property {
	/// Name of the element; this can either be the text of a simple UI element like a [`crate::Role::Button`],
	/// but it could also be alternative text via [`aria-label`](https://www.w3.org/TR/wai-aria/#aria-label).
	Name(String),
	/// The extended description of an item (usually via [`aria-describedby`](https://www.w3.org/TR/wai-aria/#aria-describedby)).
	Description(String),
	/// The [ARIA role](https://www.w3.org/TR/wai-aria/#roles) of a given item.
	Role(crate::Role),
	/// Parent of the item in a hierarchical tree.
	Parent(ObjectRefOwned),
	/// A description of the table as a whole: in HTML this is achieved via the
	/// `<table><caption>VALUE_HERE</caption>...</table>` pattern
	TableCaption(String),
	/// Similar to [`Self::TableColumnHeader`] except it's the attached description instead of the
	/// data in the header.
	TableColumnDescription(String),
	/// A column header: in HTML this is accomplished with the use of `<th>` in an aligned column with a given `<td>` cell element
	TableColumnHeader(String),
	/// Similar to [`Self::TableRowHeader`] except it's the attached description instead of the
	/// data in the header.
	TableRowDescription(String),
	/// Row header: in HTML this is accomplished with the use of `<th scope="row">` at the beginning of a `<tr>`
	TableRowHeader(String),
	/// The table summary is a shorter description of the table. In HTML this would be accomplished
	/// with the [figure/figcaption pattern](https://www.w3.org/WAI/tutorials/tables/caption-summary/#using-the-figure-element-to-mark-up-a-table-summary)
	TableSummary(String),
	/// The attached help text of the item.
	HelpText(String),
	/// Any other attribute not explicitly laid out above.
	Other((String, OwnedValue)),
}

impl Clone for Property {
	fn clone(&self) -> Self {
		match self {
			Property::Name(name) => Self::Name(name.clone()),
			Property::Description(description) => Self::Description(description.clone()),
			Property::Role(role) => Self::Role(*role),
			Property::Parent(parent) => Self::Parent(parent.clone()),
			Property::TableCaption(table_caption) => Self::TableCaption(table_caption.clone()),
			Property::TableColumnDescription(table_column_description) => {
				Self::TableColumnDescription(table_column_description.clone())
			}
			Property::TableColumnHeader(table_column_header) => {
				Self::TableColumnHeader(table_column_header.clone())
			}
			Property::TableRowDescription(table_row_description) => {
				Self::TableRowDescription(table_row_description.clone())
			}
			Property::TableRowHeader(table_row_header) => {
				Self::TableRowHeader(table_row_header.clone())
			}
			Property::TableSummary(table_summary) => Self::TableSummary(table_summary.clone()),
      Property::HelpText(help_text) => Self::HelpText(help_text.clone()),
			Property::Other((property, value)) => Self::Other((
				property.clone(),
				value
					.try_clone()
					.expect("Could not clone 'value'.  Since properties are not known to carry files, we do not expect to exceed open file limit."),
			)),
		}
	}
}

impl Default for Property {
	fn default() -> Self {
		Self::Other((String::default(), u64::default().into()))
	}
}

impl TryFrom<EventBody<'_>> for Property {
	type Error = AtspiError;

	fn try_from(mut body: EventBody<'_>) -> Result<Self, Self::Error> {
		let property = body.kind();

		match property {
			ACCESSIBLE_NAME_PROPERTY_NAME => Ok(Self::Name(
				body.take_any_data()
					.try_into()
					.map_err(|_| AtspiError::ParseError(ACCESSIBLE_NAME_PROPERTY_NAME))?,
			)),
			ACCESSIBLE_DESCRIPTION_PROPERTY_NAME => Ok(Self::Description(
				body.take_any_data()
					.try_into()
					.map_err(|_| AtspiError::ParseError(ACCESSIBLE_DESCRIPTION_PROPERTY_NAME))?,
			)),
			ACCESSIBLE_ROLE_PROPERTY_NAME => Ok(Self::Role({
				let role_int: u32 = body
					.any_data()
					.try_into()
					.map_err(|_| AtspiError::ParseError(ACCESSIBLE_ROLE_PROPERTY_NAME))?;
				let role: crate::Role = crate::Role::try_from(role_int)
					.map_err(|_| AtspiError::ParseError("accessible-role"))?;
				role
			})),
			ACCESSIBLE_PARENT_PROPERTY_NAME => Ok(Self::Parent(
				body.take_any_data()
					.try_into()
					.map_err(|_| AtspiError::ParseError(ACCESSIBLE_PARENT_PROPERTY_NAME))?,
			)),
			ACCESSIBLE_TABLE_CAPTION_PROPERTY_NAME => Ok(Self::TableCaption(
				body.take_any_data()
					.try_into()
					.map_err(|_| AtspiError::ParseError(ACCESSIBLE_TABLE_CAPTION_PROPERTY_NAME))?,
			)),
			ACCESSIBLE_TABLE_COLUMN_DESCRIPTION_PROPERTY_NAME => {
				Ok(Self::TableColumnDescription(body.take_any_data().try_into().map_err(|_| {
					AtspiError::ParseError(ACCESSIBLE_TABLE_COLUMN_DESCRIPTION_PROPERTY_NAME)
				})?))
			}
			ACCESSIBLE_TABLE_COLUMN_HEADER_PROPERTY_NAME => {
				Ok(Self::TableColumnHeader(body.take_any_data().try_into().map_err(|_| {
					AtspiError::ParseError(ACCESSIBLE_TABLE_COLUMN_HEADER_PROPERTY_NAME)
				})?))
			}
			ACCESSIBLE_TABLE_ROW_DESCRIPTION_PROPERTY_NAME => {
				Ok(Self::TableRowDescription(body.take_any_data().try_into().map_err(|_| {
					AtspiError::ParseError(ACCESSIBLE_TABLE_ROW_DESCRIPTION_PROPERTY_NAME)
				})?))
			}
			ACCESSIBLE_TABLE_ROW_HEADER_PROPERTY_NAME => {
				Ok(Self::TableRowHeader(body.take_any_data().try_into().map_err(|_| {
					AtspiError::ParseError(ACCESSIBLE_TABLE_ROW_HEADER_PROPERTY_NAME)
				})?))
			}
			ACCESSIBLE_TABLE_SUMMARY_PROPERTY_NAME => Ok(Self::TableSummary(
				body.take_any_data()
					.try_into()
					.map_err(|_| AtspiError::ParseError(ACCESSIBLE_TABLE_SUMMARY_PROPERTY_NAME))?,
			)),
			ACCESSIBLE_HELP_TEXT_PROPERTY_NAME => Ok(Self::HelpText(
				body.take_any_data()
					.try_into()
					.map_err(|_| AtspiError::ParseError(ACCESSIBLE_HELP_TEXT_PROPERTY_NAME))?,
			)),
			_ => Ok(Self::Other((property.to_string(), body.take_any_data()))),
		}
	}
}

impl From<Property> for OwnedValue {
	fn from(property: Property) -> Self {
		let value = match property {
			Property::Name(name) => Value::from(name),
			Property::Description(description) => Value::from(description),
			Property::Role(role) => Value::from(role as u32),
			Property::Parent(parent) => Value::from(parent),
			Property::TableCaption(table_caption) => Value::from(table_caption),
			Property::TableColumnDescription(table_column_description) => {
				Value::from(table_column_description)
			}
			Property::TableColumnHeader(table_column_header) => Value::from(table_column_header),
			Property::TableRowDescription(table_row_description) => {
				Value::from(table_row_description)
			}
			Property::TableRowHeader(table_row_header) => Value::from(table_row_header),
			Property::TableSummary(table_summary) => Value::from(table_summary),
			Property::HelpText(help_text) => Value::from(help_text),
			Property::Other((_, value)) => value.into(),
		};
		value.try_into().expect("Should succeed as there are no borrowed file descriptors involved that could, potentially, exceed the open file limit when converted to OwnedValue")
	}
}

#[cfg(test)]
mod test_property {
	use crate::events::object::{Property, PropertyChangeEvent};
	use crate::events::{EventBody, EventBodyOwned};
	use crate::{NonNullObjectRef, ObjectRef, Role};

	static NON_NULL_TEST_OBJECT: &NonNullObjectRef =
		&NonNullObjectRef::from_static_str_unchecked(":0.0", "/org/a11y/atspi/test/path");

	macro_rules! property_subtype_test {
		($name:ident, $key:expr, $prop:path, $val:expr) => {
			#[test]
			fn $name() {
				let prop = $prop($val);
				let prop_ev = PropertyChangeEvent {
					item: NON_NULL_TEST_OBJECT.clone().into(),
					property: $key.to_string(),
					value: prop.clone(),
				};
				let ev_body: EventBodyOwned = prop_ev.try_into().expect("Valid event body!");
				let ev: EventBody<'_> = ev_body.into();
				let prop2: Property = ev.try_into().expect("Valid Property value");
				assert_eq!(prop, prop2);
			}
		};
	}
	property_subtype_test!(
		test_prop_type_desc,
		"accessible-description",
		Property::Description,
		"Accessible description text here!".to_string()
	);
	property_subtype_test!(
		test_prop_type_name,
		"accessible-name",
		Property::Name,
		"Accessible name here!".to_string()
	);
	property_subtype_test!(test_prop_type_role, "accessible-role", Property::Role, Role::Invalid);
	property_subtype_test!(
		test_prop_type_parent,
		"accessible-parent",
		Property::Parent,
		ObjectRef::from_static_str_unchecked(":420.69", "/fake/a11y/addr").into()
	);
	property_subtype_test!(
		test_prop_type_table_caption,
		"accessible-table-caption",
		Property::TableCaption,
		"Accessible table description here".to_string()
	);
	property_subtype_test!(
		test_prop_type_table_cd,
		"accessible-table-column-description",
		Property::TableColumnDescription,
		"Accessible table column description here!".to_string()
	);
	property_subtype_test!(
		test_prop_type_table_ch,
		"accessible-table-column-header",
		Property::TableColumnHeader,
		"Accessible table column header here!".to_string()
	);
	property_subtype_test!(
		test_prop_type_table_rd,
		"accessible-table-row-description",
		Property::TableRowDescription,
		"Accessible table row description here!".to_string()
	);
	property_subtype_test!(
		test_prop_type_table_rh,
		"accessible-table-row-header",
		Property::TableRowHeader,
		"Accessible table row header here!".to_string()
	);
	property_subtype_test!(
		test_prop_help_text,
		"accessible-help-text",
		Property::HelpText,
		"Accessible help text here!".to_string()
	);
}

/// An event triggered when the visual bounds for an item have changed.
/// This usually happens either:
///
/// 1. due to a re-draw on a window whose size has changed and dynamically adjusted said item's visual size, or
/// 2. content within the bounds of said item has changed to change its size.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct BoundsChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(BoundsChangedEvent<'_>);

/// A link has been selected.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct LinkSelectedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(LinkSelectedEvent<'_>);

/// A state of an object has been modified.
/// A [`State`] can be added or removed from any [`crate::ObjectRef`].
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct StateChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// The state to be enabled/disabled.
	pub state: State,
	/// Whether the state was enabled or disabled.
	#[serde(with = "i32_bool_conversion")]
	pub enabled: bool,
}

impl_event_type_properties_for_event!(StateChangedEvent<'_>);

mod i32_bool_conversion {
	use serde::{Deserialize, Deserializer, Serializer};
	/// Convert an integer flag to a boolean.
	/// returns true if value is more than 0, otherwise false
	pub fn deserialize<'de, D>(de: D) -> Result<bool, D::Error>
	where
		D: Deserializer<'de>,
	{
		let int: i32 = Deserialize::deserialize(de)?;
		Ok(int > 0)
	}

	/// Convert a boolean flag to an integer.
	/// returns 0 if false and 1 if true
	// TODO: Will the world REALLY fall apart if we were not to use a reference here?
	// In other words, see if &bool can be replaced with bool.
	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn serialize<S>(b: &bool, ser: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let val: i32 = (*b).into();
		ser.serialize_i32(val)
	}
}

/// A child of `item` has been added or removed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ChildrenChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// The [`crate::Operation`] being performed.
	pub operation: crate::Operation,
	/// Index to remove from/add to.
	pub index_in_parent: i32,
	/// A reference to the new child.
	pub child: ObjectRefOwned,
}

impl_event_type_properties_for_event!(ChildrenChangedEvent<'_>);

/// A change in whether a particular item is visible or invisible (but still present).
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct VisibleDataChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(VisibleDataChangedEvent<'_>);

/// The selection of this item has changed.
/// For example: when a selection from a series of checkboxes is changed, this will change the state of the child, _and_ cause a [`SelectionChangedEvent`] on the parent.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct SelectionChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(SelectionChangedEvent<'_>);

/// An event sent when the method of selecting items in a list/set of options changes.
/// Also see: <https://docs.gtk.org/gtk4//method.GridView.set_model.html>
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ModelChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ModelChangedEvent<'_>);

/// An event fired when the focus has moved within a tree.
/// The parent: `item` and descendant (may not be a direct child): `descebdant` are both referenced for convenience.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ActiveDescendantChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// The descendant which is now the active one.
	pub descendant: ObjectRefOwned,
}

impl_event_type_properties_for_event!(ActiveDescendantChangedEvent<'_>);

/// An announcement with a defined text string and an [ARIA politeness level](https://www.w3.org/TR/2009/WD-wai-aria-20091215/states_and_properties#aria-live).
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct AnnouncementEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// Text of the announcement.
	pub text: String,
	/// Politeness level.
	pub live: crate::Politeness,
}

impl_event_type_properties_for_event!(AnnouncementEvent<'_>);

/// Signal that some attribute of an object (usually styling) has changed.
/// This event does not encode _what_ has changed about the attributes, merely that they have
/// changed.
///
/// To query the updated information, use `atspi_proxies::AccessibleProxy`'s `get_attribute` method.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct AttributesChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(AttributesChangedEvent<'_>);

/// A row has been added to a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct RowInsertedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(RowInsertedEvent<'_>);

/// A row has been moved within a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct RowReorderedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(RowReorderedEvent<'_>);

/// A row has been deleted from a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct RowDeletedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(RowDeletedEvent<'_>);

/// A column has been added to a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ColumnInsertedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ColumnInsertedEvent<'_>);

/// A column has been re-ordered within a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ColumnReorderedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ColumnReorderedEvent<'_>);

/// A column has been removed from a table.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct ColumnDeletedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(ColumnDeletedEvent<'_>);

/// The bounds of a piece of text have changed.
/// This event does _not_ specify what the new bounds are; it is only to notify an AT that the bounds have changed.
/// To query information about the new state of the selection, use `atspi_proxies::TextProxy`'s `get_bounded_ranges` function.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct TextBoundsChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(TextBoundsChangedEvent<'_>);

/// The user's selection of a piece of text has changed.
/// This event does _not_ specify what the new selection is, nor its indecies; it is only to notify an AT that the selection has changed.
/// To query information about the new state of the selection, use `atspi_proxies::TextProxy`'s methods.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct TextSelectionChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(TextSelectionChangedEvent<'_>);

/// Text has changed within the UI element `item`.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct TextChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// The [`crate::Operation`] being performed.
	pub operation: crate::Operation,
	/// starting index of the insertion/deletion
	///
	/// NOTE: This gives the Unicode index (not the byte index). I.e., it groups unicode sequences
	/// into one character.
	/// Always use the appropriate insertion methods to deal with this, i.e., do not use
	/// [`String::insert_str`].
	pub start_pos: i32,
	/// length of the insertion/deletion
	///
	/// NOTE: This gives the unicode length (not the byte length).
	pub length: i32,
	/// the text being inserted/deleted
	pub text: String,
}

impl_event_type_properties_for_event!(TextChangedEvent<'_>);

/// Signal that some attributes about the text (usually styling) have changed.
/// This event does not encode _what_ has changed about the attributes, merely that they have
/// changed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct TextAttributesChangedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
}

impl_event_type_properties_for_event!(TextAttributesChangedEvent<'_>);

/// The caret of the user also known as a cursor (not to be confused with mouse pointer) has changed position.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash)]
pub struct TextCaretMovedEvent<'a> {
	/// The [`crate::NonNullObjectRef`] which the event applies to.
	#[serde(borrow)]
	pub item: NonNullObjectRef<'a>,
	/// New position of the caret.
	/// NOTE: this provide the Unicode index (not the byte index) and therefore when referencing
	/// locations in a string, you should be using the [`std::str::Chars`] iterator, and not use
	/// anything like [`str::get`] (as this uses the byte index).
	///
	/// See also: [`TextChangedEvent`].
	pub position: i32,
}

impl_event_type_properties_for_event!(TextCaretMovedEvent<'_>);

impl_member_interface_registry_string_and_match_rule_for_event!(
	PropertyChangeEvent<'_>,
	"PropertyChange",
	"org.a11y.atspi.Event.Object",
	"object:property-change",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for PropertyChangeEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		let property: String = body.take_kind();
		let value: Property = body.try_into()?;
		Ok(Self { item: item.into_owned(), property, value })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		let copy = self.clone();
		EventBodyOwned::from(copy).into()
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	BoundsChangedEvent<'_>,
	"BoundsChanged",
	"org.a11y.atspi.Event.Object",
	"object:bounds-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='BoundsChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LinkSelectedEvent<'_>,
	"LinkSelected",
	"org.a11y.atspi.Event.Object",
	"object:link-selected",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='LinkSelected'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	StateChangedEvent<'_>,
	"StateChanged",
	"org.a11y.atspi.Event.Object",
	"object:state-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for StateChangedEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		let body: Self::Body<'_> = body.deserialize_unchecked()?;
		Ok(Self { item, state: body.kind().into(), enabled: body.detail1() > 0 })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		let copy = self.clone();
		copy.into()
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	ChildrenChangedEvent<'_>,
	"ChildrenChanged",
	"org.a11y.atspi.Event.Object",
	"object:children-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='ChildrenChanged'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for ChildrenChangedEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		// Needs `traits::MessageConversion::from_message_unchecked_parts` to borrow `DBusBody`
		// to achieve zero-copy. Without that borrow, DBusBody is put on the stack for
		// this method and dropped at the end.
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self {
			item: item.into_owned(),
			operation: body.kind().parse()?,
			index_in_parent: body.detail1(),
			child: body.take_any_data().try_into()?,
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self.clone()).into()
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	VisibleDataChangedEvent<'_>,
	"VisibleDataChanged",
	"org.a11y.atspi.Event.Object",
	"object:visible-data-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='VisibleDataChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	SelectionChangedEvent<'_>,
	"SelectionChanged",
	"org.a11y.atspi.Event.Object",
	"object:selection-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='SelectionChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ModelChangedEvent<'_>,
	"ModelChanged",
	"org.a11y.atspi.Event.Object",
	"object:model-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='ModelChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ActiveDescendantChangedEvent<'_>,
	"ActiveDescendantChanged",
	"org.a11y.atspi.Event.Object",
	"object:active-descendant-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='ActiveDescendantChanged'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for ActiveDescendantChangedEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		// Needs `traits::MessageConversion::from_message_unchecked_parts` to borrow `DBusBody`
		// to achieve zero-copy. Without that borrow, DBusBody is put on the stack for
		// this method and dropped at the end.
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item: item.into_owned(), descendant: body.take_any_data().try_into()? })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self.clone()).into()
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	AnnouncementEvent<'_>,
	"Announcement",
	"org.a11y.atspi.Event.Object",
	"object:announcement",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for AnnouncementEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		// Needs `traits::MessageConversion::from_message_unchecked_parts` to borrow `DBusBody`
		// to achieve zero-copy. Without that borrow, DBusBody is put on the stack for
		// this method and dropped at the end.
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self {
			item: item.into_owned(),
			text: body
				.take_any_data()
				.try_into()
				.map_err(|_| AtspiError::Conversion("text"))?,
			live: body.detail1().try_into()?,
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self.clone()).into()
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	AttributesChangedEvent<'_>,
	"AttributesChanged",
	"org.a11y.atspi.Event.Object",
	"object:attributes-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RowInsertedEvent<'_>,
	"RowInserted",
	"org.a11y.atspi.Event.Object",
	"object:row-inserted",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='RowInserted'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RowReorderedEvent<'_>,
	"RowReordered",
	"org.a11y.atspi.Event.Object",
	"object:row-reordered",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='RowReordered'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RowDeletedEvent<'_>,
	"RowDeleted",
	"org.a11y.atspi.Event.Object",
	"object:row-deleted",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='RowDeleted'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ColumnInsertedEvent<'_>,
	"ColumnInserted",
	"org.a11y.atspi.Event.Object",
	"object:column-inserted",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnInserted'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ColumnReorderedEvent<'_>,
	"ColumnReordered",
	"org.a11y.atspi.Event.Object",
	"object:column-reordered",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnReordered'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ColumnDeletedEvent<'_>,
	"ColumnDeleted",
	"org.a11y.atspi.Event.Object",
	"object:column-deleted",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnDeleted'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	TextBoundsChangedEvent<'_>,
	"TextBoundsChanged",
	"org.a11y.atspi.Event.Object",
	"object:text-bounds-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='TextBoundsChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	TextSelectionChangedEvent<'_>,
	"TextSelectionChanged",
	"org.a11y.atspi.Event.Object",
	"object:text-selection-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='TextSelectionChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	TextChangedEvent<'_>,
	"TextChanged",
	"org.a11y.atspi.Event.Object",
	"object:text-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='TextChanged'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for TextChangedEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'_>,
		dbus_body: DbusBody,
	) -> Result<Self, AtspiError> {
		let mut body = dbus_body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self {
			item: item.into_owned(),
			operation: body.kind().parse()?,
			start_pos: body.detail1(),
			length: body.detail2(),
			text: body.take_any_data().try_into()?,
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self.clone()).into()
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	TextAttributesChangedEvent<'_>,
	"TextAttributesChanged",
	"org.a11y.atspi.Event.Object",
	"object:text-attributes-changed",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='TextAttributesChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	TextCaretMovedEvent<'_>,
	"TextCaretMoved",
	"org.a11y.atspi.Event.Object",
	"object:text-caret-moved",
	"type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved'"
);

#[cfg(feature = "zbus")]
impl<'a> MessageConversion<'a> for TextCaretMovedEvent<'a> {
	type Body<'msg>
		= EventBody<'msg>
	where
		Self: 'msg;

	fn from_message_unchecked_parts(
		item: NonNullObjectRef<'a>,
		body: DbusBody,
	) -> Result<Self, AtspiError> {
		// Deserialize the body with a short lived anonymous lifetime,
		// we don't care about its name - we are only interested in a `Copy` type.
		let body_data = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item, position: body_data.detail1() })
	}

	fn from_message_unchecked(msg: &'a zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item: NonNullObjectRef<'_> = header.try_into()?;
		let dbus_body = msg.body();
		Self::from_message_unchecked_parts(item.into_owned(), dbus_body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self.clone()).into()
	}
}

event_test_cases!(PropertyChangeEvent, [property, value], Auto);
impl_to_dbus_message!(PropertyChangeEvent<'_>);
impl_from_dbus_message!(PropertyChangeEvent<'_>);
impl_event_properties!(PropertyChangeEvent<'_>);

impl From<PropertyChangeEvent<'_>> for EventBodyOwned {
	fn from(event: PropertyChangeEvent) -> Self {
		EventBodyOwned { kind: event.property, any_data: event.value.into(), ..Default::default() }
	}
}

impl From<&PropertyChangeEvent<'_>> for EventBodyOwned {
	fn from(event: &PropertyChangeEvent) -> Self {
		EventBodyOwned {
			kind: event.property.clone(),
			any_data: event.value.clone().into(),
			..Default::default()
		}
	}
}

impl From<PropertyChangeEvent<'_>> for EventBody<'_> {
	fn from(event: PropertyChangeEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

event_test_cases!(BoundsChangedEvent);
impl_to_dbus_message!(BoundsChangedEvent<'_>);
impl_from_dbus_message!(BoundsChangedEvent<'_>);
impl_event_properties!(BoundsChangedEvent<'_>);
impl_from_non_null_object_ref!(BoundsChangedEvent<'_>);

event_test_cases!(LinkSelectedEvent);
impl_to_dbus_message!(LinkSelectedEvent<'_>);
impl_from_dbus_message!(LinkSelectedEvent<'_>);
impl_event_properties!(LinkSelectedEvent<'_>);
impl_from_non_null_object_ref!(LinkSelectedEvent<'_>);

event_test_cases!(StateChangedEvent, [state, enabled], Auto);
impl_to_dbus_message!(StateChangedEvent<'_>);
impl_from_dbus_message!(StateChangedEvent<'_>);
impl_event_properties!(StateChangedEvent<'_>);

impl From<StateChangedEvent<'_>> for EventBodyOwned {
	fn from(event: StateChangedEvent) -> Self {
		EventBodyOwned {
			kind: event.state.to_string(),
			detail1: event.enabled.into(),
			..Default::default()
		}
	}
}

impl From<&StateChangedEvent<'_>> for EventBodyOwned {
	fn from(event: &StateChangedEvent) -> Self {
		EventBodyOwned {
			kind: event.state.to_string(),
			detail1: event.enabled.into(),
			..Default::default()
		}
	}
}

impl From<StateChangedEvent<'_>> for EventBody<'_> {
	fn from(event: StateChangedEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

event_test_cases!(ChildrenChangedEvent, [operation, index_in_parent, child], Auto);
impl_to_dbus_message!(ChildrenChangedEvent<'_>);
impl_from_dbus_message!(ChildrenChangedEvent<'_>);
impl_event_properties!(ChildrenChangedEvent<'_>);

impl From<ChildrenChangedEvent<'_>> for EventBodyOwned {
	fn from(event: ChildrenChangedEvent) -> Self {
		EventBodyOwned {
			kind: event.operation.to_string(),
			detail1: event.index_in_parent,

			// `OwnedValue` is constructed from the `crate::ObjectRef`
			// Only way to fail is to convert a `Fd` into an `OwnedValue`.
			// Therefore, this is safe.
			any_data: Value::from(event.child)
				.try_into()
				.expect("Failed to convert child to OwnedValue"),
			..Default::default()
		}
	}
}

impl From<&ChildrenChangedEvent<'_>> for EventBodyOwned {
	fn from(event: &ChildrenChangedEvent) -> Self {
		EventBodyOwned {
			kind: event.operation.to_string(),
			detail1: event.index_in_parent,
			detail2: i32::default(),
			// `OwnedValue` is constructed from the `crate::ObjectRef`
			// Only path to fail is to convert a `Fd` into an `OwnedValue`.
			// Therefore, this is safe.
			any_data: Value::from(event.child.clone())
				.try_into()
				.expect("ObjectRef should convert to OwnedValue without error"),
			properties: super::event_body::Properties,
		}
	}
}

impl From<ChildrenChangedEvent<'_>> for EventBody<'_> {
	fn from(event: ChildrenChangedEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

event_test_cases!(VisibleDataChangedEvent);
impl_to_dbus_message!(VisibleDataChangedEvent<'_>);
impl_from_dbus_message!(VisibleDataChangedEvent<'_>);
impl_event_properties!(VisibleDataChangedEvent<'_>);
impl_from_non_null_object_ref!(VisibleDataChangedEvent<'_>);

event_test_cases!(SelectionChangedEvent);
impl_to_dbus_message!(SelectionChangedEvent<'_>);
impl_from_dbus_message!(SelectionChangedEvent<'_>);
impl_event_properties!(SelectionChangedEvent<'_>);
impl_from_non_null_object_ref!(SelectionChangedEvent<'_>);

event_test_cases!(ModelChangedEvent);
impl_to_dbus_message!(ModelChangedEvent<'_>);
impl_from_dbus_message!(ModelChangedEvent<'_>);
impl_event_properties!(ModelChangedEvent<'_>);
impl_from_non_null_object_ref!(ModelChangedEvent<'_>);

event_test_cases!(ActiveDescendantChangedEvent);
impl_to_dbus_message!(ActiveDescendantChangedEvent<'_>);
impl_from_dbus_message!(ActiveDescendantChangedEvent<'_>);
impl_event_properties!(ActiveDescendantChangedEvent<'_>);

impl From<ActiveDescendantChangedEvent<'_>> for EventBodyOwned {
	fn from(event: ActiveDescendantChangedEvent) -> Self {
		EventBodyOwned {
			// `OwnedValue` is constructed from the `crate::ObjectRef`
			// Only way to fail is to convert a Fd into an `OwnedValue`.
			// Therefore, this is safe.
			any_data: Value::from(event.descendant)
				.try_to_owned()
				.expect("Failed to convert descendant to OwnedValue"),
			..Default::default()
		}
	}
}

event_test_cases!(AnnouncementEvent, [text, live], Auto);
impl_to_dbus_message!(AnnouncementEvent<'_>);
impl_from_dbus_message!(AnnouncementEvent<'_>);
impl_event_properties!(AnnouncementEvent<'_>);
impl From<AnnouncementEvent<'_>> for EventBodyOwned {
	fn from(event: AnnouncementEvent) -> Self {
		EventBodyOwned {
			detail1: event.live as i32,
			// `OwnedValue` is constructed from `String`
			// Therefore, this is safe.
			any_data: Value::from(event.text)
				.try_to_owned()
				.expect("Failed to convert text to OwnedValue"),
			..Default::default()
		}
	}
}

event_test_cases!(AttributesChangedEvent);
impl_to_dbus_message!(AttributesChangedEvent<'_>);
impl_from_dbus_message!(AttributesChangedEvent<'_>);
impl_event_properties!(AttributesChangedEvent<'_>);
impl_from_non_null_object_ref!(AttributesChangedEvent<'_>);

event_test_cases!(RowInsertedEvent);
impl_to_dbus_message!(RowInsertedEvent<'_>);
impl_from_dbus_message!(RowInsertedEvent<'_>);
impl_event_properties!(RowInsertedEvent<'_>);
impl_from_non_null_object_ref!(RowInsertedEvent<'_>);

event_test_cases!(RowReorderedEvent);
impl_to_dbus_message!(RowReorderedEvent<'_>);
impl_from_dbus_message!(RowReorderedEvent<'_>);
impl_event_properties!(RowReorderedEvent<'_>);
impl_from_non_null_object_ref!(RowReorderedEvent<'_>);

event_test_cases!(RowDeletedEvent);
impl_to_dbus_message!(RowDeletedEvent<'_>);
impl_from_dbus_message!(RowDeletedEvent<'_>);
impl_event_properties!(RowDeletedEvent<'_>);
impl_from_non_null_object_ref!(RowDeletedEvent<'_>);

event_test_cases!(ColumnInsertedEvent);
impl_to_dbus_message!(ColumnInsertedEvent<'_>);
impl_from_dbus_message!(ColumnInsertedEvent<'_>);
impl_event_properties!(ColumnInsertedEvent<'_>);
impl_from_non_null_object_ref!(ColumnInsertedEvent<'_>);

event_test_cases!(ColumnReorderedEvent);
impl_to_dbus_message!(ColumnReorderedEvent<'_>);
impl_from_dbus_message!(ColumnReorderedEvent<'_>);
impl_event_properties!(ColumnReorderedEvent<'_>);
impl_from_non_null_object_ref!(ColumnReorderedEvent<'_>);

event_test_cases!(ColumnDeletedEvent);
impl_to_dbus_message!(ColumnDeletedEvent<'_>);
impl_from_dbus_message!(ColumnDeletedEvent<'_>);
impl_event_properties!(ColumnDeletedEvent<'_>);
impl_from_non_null_object_ref!(ColumnDeletedEvent<'_>);

event_test_cases!(TextBoundsChangedEvent);
impl_to_dbus_message!(TextBoundsChangedEvent<'_>);
impl_from_dbus_message!(TextBoundsChangedEvent<'_>);
impl_event_properties!(TextBoundsChangedEvent<'_>);
impl_from_non_null_object_ref!(TextBoundsChangedEvent<'_>);

event_test_cases!(TextSelectionChangedEvent);
impl_to_dbus_message!(TextSelectionChangedEvent<'_>);
impl_from_dbus_message!(TextSelectionChangedEvent<'_>);
impl_event_properties!(TextSelectionChangedEvent<'_>);
impl_from_non_null_object_ref!(TextSelectionChangedEvent<'_>);

event_test_cases!(TextChangedEvent, [operation, start_pos, length, text], Auto);
impl_to_dbus_message!(TextChangedEvent<'_>);
impl_from_dbus_message!(TextChangedEvent<'_>);
impl_event_properties!(TextChangedEvent<'_>);

impl From<TextChangedEvent<'_>> for EventBodyOwned {
	fn from(event: TextChangedEvent) -> Self {
		EventBodyOwned {
			kind: event.operation.to_string(),
			detail1: event.start_pos,
			detail2: event.length,
			// `OwnedValue` is constructed from a `String`
			// Therefore, this is safe.
			any_data: Value::from(event.text)
				.try_to_owned()
				.expect("Failed to convert child to OwnedValue"),
			..Default::default()
		}
	}
}

event_test_cases!(TextAttributesChangedEvent);
impl_to_dbus_message!(TextAttributesChangedEvent<'_>);
impl_from_dbus_message!(TextAttributesChangedEvent<'_>);
impl_event_properties!(TextAttributesChangedEvent<'_>);
impl_from_non_null_object_ref!(TextAttributesChangedEvent<'_>);

event_test_cases!(TextCaretMovedEvent, [position], Auto);
impl_to_dbus_message!(TextCaretMovedEvent<'_>);
impl_from_dbus_message!(TextCaretMovedEvent<'_>);
impl_event_properties!(TextCaretMovedEvent<'_>);
impl From<TextCaretMovedEvent<'_>> for EventBodyOwned {
	fn from(event: TextCaretMovedEvent) -> Self {
		EventBodyOwned { detail1: event.position, ..Default::default() }
	}
}

impl_msg_conversion_ext_for_target_type!(
	AnnouncementEvent<'_>,
	ActiveDescendantChangedEvent<'_>,
	PropertyChangeEvent<'_>,
	BoundsChangedEvent<'_>,
	LinkSelectedEvent<'_>,
	StateChangedEvent<'_>,
	ChildrenChangedEvent<'_>,
	VisibleDataChangedEvent<'_>,
	SelectionChangedEvent<'_>,
	ModelChangedEvent<'_>,
	AttributesChangedEvent<'_>,
	RowInsertedEvent<'_>,
	RowReorderedEvent<'_>,
	RowDeletedEvent<'_>,
	ColumnInsertedEvent<'_>,
	ColumnReorderedEvent<'_>,
	ColumnDeletedEvent<'_>,
	TextBoundsChangedEvent<'_>,
	TextSelectionChangedEvent<'_>,
	TextChangedEvent<'_>,
	TextAttributesChangedEvent<'_>,
	TextCaretMovedEvent<'_>,
);

impl_msg_conversion_for_types_built_from_object_ref!(
	BoundsChangedEvent<'_>,
	LinkSelectedEvent<'_>,
	VisibleDataChangedEvent<'_>,
	SelectionChangedEvent<'_>,
	ModelChangedEvent<'_>,
	AttributesChangedEvent<'_>,
	RowInsertedEvent<'_>,
	RowReorderedEvent<'_>,
	RowDeletedEvent<'_>,
	ColumnInsertedEvent<'_>,
	ColumnReorderedEvent<'_>,
	ColumnDeletedEvent<'_>,
	TextBoundsChangedEvent<'_>,
	TextSelectionChangedEvent<'_>,
	TextAttributesChangedEvent<'_>,
);

impl_test_event!(
	BoundsChangedEvent<'_>,
	LinkSelectedEvent<'_>,
	VisibleDataChangedEvent<'_>,
	SelectionChangedEvent<'_>,
	ModelChangedEvent<'_>,
	AttributesChangedEvent<'_>,
	RowInsertedEvent<'_>,
	RowReorderedEvent<'_>,
	RowDeletedEvent<'_>,
	ColumnInsertedEvent<'_>,
	ColumnReorderedEvent<'_>,
	ColumnDeletedEvent<'_>,
	TextBoundsChangedEvent<'_>,
	TextSelectionChangedEvent<'_>,
	TextAttributesChangedEvent<'_>,
	AnnouncementEvent<'_> { text, live },
	ActiveDescendantChangedEvent<'_> { descendant },
	TextCaretMovedEvent<'_> { position },
	TextChangedEvent<'_> { operation, start_pos, length, text },
	PropertyChangeEvent<'_> { property, value },
	StateChangedEvent<'_> { state, enabled },
	ChildrenChangedEvent<'_> { operation, index_in_parent, child },
);
