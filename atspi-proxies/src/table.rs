//! # `TableProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Table`
//! interface.
//!
//! The `Table` interface provides methods to interact with two-dimensional
//! grids and table-like UI elements. This includes querying rows and columns,
//! accessing cells at specific coordinates ([`get_accessible_at`]), retrieving
//! row/column headers, and managing selections (such as [`get_selected_rows`]
//! and [`get_selected_columns`]).
//!
//! ## Defaults
//!
//! The `Table` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain a `TableProxy`
//!
//! There are two idiomatic ways to obtain a `TableProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a tabular node, you can safely
//! query and convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let table = proxies.table().await?;
//! ```
//!
//! All proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting any P2P configuration if applicable.
//!
//! ### 2. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,ignore
//! let table = TableProxy::builder(&connection)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`get_accessible_at`]: TableProxy#method.get_accessible_at
//! [`get_selected_rows`]: TableProxy#method.get_selected_rows
//! [`get_selected_columns`]: TableProxy#method.get_selected_columns
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

use atspi_common::object_ref::ObjectRefOwned;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(interface = "org.a11y.atspi.Table", assume_defaults = false)]
pub trait Table {
	/// `AddColumnSelection` method
	fn add_column_selection(&self, column: i32) -> zbus::Result<bool>;

	/// `AddRowSelection` method
	fn add_row_selection(&self, row: i32) -> zbus::Result<bool>;

	/// `GetAccessibleAt` method
	fn get_accessible_at(&self, row: i32, column: i32) -> zbus::Result<ObjectRefOwned>;

	/// `GetColumnAtIndex` method
	fn get_column_at_index(&self, index: i32) -> zbus::Result<i32>;

	/// `GetColumnDescription` method
	fn get_column_description(&self, column: i32) -> zbus::Result<String>;

	/// `GetColumnExtentAt` method
	fn get_column_extent_at(&self, row: i32, column: i32) -> zbus::Result<i32>;

	/// `GetColumnHeader` method
	fn get_column_header(&self, column: i32) -> zbus::Result<ObjectRefOwned>;

	/// `GetIndexAt` method
	fn get_index_at(&self, row: i32, column: i32) -> zbus::Result<i32>;

	/// `GetRowAtIndex` method
	fn get_row_at_index(&self, index: i32) -> zbus::Result<i32>;

	/// `GetRowColumnExtentsAtIndex` method
	fn get_row_column_extents_at_index(
		&self,
		index: i32,
	) -> zbus::Result<(bool, i32, i32, i32, i32, bool)>;

	/// `GetRowDescription` method
	fn get_row_description(&self, row: i32) -> zbus::Result<String>;

	/// `GetRowExtentAt` method
	fn get_row_extent_at(&self, row: i32, column: i32) -> zbus::Result<i32>;

	/// `GetRowHeader` method
	fn get_row_header(&self, row: i32) -> zbus::Result<ObjectRefOwned>;

	/// `GetSelectedColumns` method
	fn get_selected_columns(&self) -> zbus::Result<Vec<i32>>;

	/// `GetSelectedRows` method
	fn get_selected_rows(&self) -> zbus::Result<Vec<i32>>;

	/// `IsColumnSelected` method
	fn is_column_selected(&self, column: i32) -> zbus::Result<bool>;

	/// `IsRowSelected` method
	fn is_row_selected(&self, row: i32) -> zbus::Result<bool>;

	/// `IsSelected` method
	fn is_selected(&self, row: i32, column: i32) -> zbus::Result<bool>;

	/// `RemoveColumnSelection` method
	fn remove_column_selection(&self, column: i32) -> zbus::Result<bool>;

	/// `RemoveRowSelection` method
	fn remove_row_selection(&self, row: i32) -> zbus::Result<bool>;

	/// `Caption` property
	#[zbus(property)]
	fn caption(&self) -> zbus::Result<ObjectRefOwned>;

	/// `NColumns` property
	#[zbus(property, name = "NColumns")]
	fn n_columns(&self) -> zbus::Result<i32>;

	/// `NRows` property
	#[zbus(property, name = "NRows")]
	fn n_rows(&self) -> zbus::Result<i32>;

	/// `NSelectedColumns` property
	#[zbus(property, name = "NSelectedColumns")]
	fn n_selected_columns(&self) -> zbus::Result<i32>;

	/// `NSelectedRows` property
	#[zbus(property, name = "NSelectedRows")]
	fn n_selected_rows(&self) -> zbus::Result<i32>;

	/// `Summary` property
	#[zbus(property)]
	fn summary(&self) -> zbus::Result<ObjectRefOwned>;
}
