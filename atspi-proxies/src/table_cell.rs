//! # `TableCellProxy`
//!
//! `org.a11y.atspi.TableCell` provides methods to interact with
//! table cell-like UI elements.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Table" may be implemented for individual nodes
//! in the application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this `TableProxy` is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.

use atspi_common::object_ref::ObjectRefOwned;

#[zbus::proxy(interface = "org.a11y.atspi.TableCell", assume_defaults = false)]
pub trait TableCell {
	/// `GetColumnHeaderCells` method
	fn get_column_header_cells(&self) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// `GetRowColumnSpan` method
	fn get_row_column_span(&self) -> zbus::Result<(bool, i32, i32, i32, i32)>;

	/// `GetRowHeaderCells` method
	fn get_row_header_cells(&self) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// `ColumnSpan` property
	#[zbus(property)]
	fn column_span(&self) -> zbus::Result<i32>;

	/// `Position` property
	#[zbus(property)]
	fn position(&self) -> zbus::Result<(i32, i32)>;

	/// `RowSpan` property
	#[zbus(property)]
	fn row_span(&self) -> zbus::Result<i32>;

	/// `Table` property
	#[zbus(property)]
	fn table(&self) -> zbus::Result<ObjectRefOwned>;
}
