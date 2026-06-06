//! # `TableCellProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.TableCell`
//! interface.
//!
//! The `TableCell` interface provides methods to interact with individual cells inside
//! a table-like element. This includes retrieving its table-relative position ([`position`]),
//! row and column spans ([`row_span`], [`column_span`]), and associated header cells
//! (via [`get_row_header_cells`] or [`get_column_header_cells`]).
//!
//! ## Defaults
//!
//! The `TableCell` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain a `TableCellProxy`
//!
//! There are two idiomatic ways to obtain a `TableCellProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] representing a cell, you can safely
//! query and convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let table_cell = proxies.table_cell().await?;
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
//! let table_cell = TableCellProxy::builder(&connection)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`position`]: TableCellProxy#method.position
//! [`row_span`]: TableCellProxy#method.row_span
//! [`column_span`]: TableCellProxy#method.column_span
//! [`get_row_header_cells`]: TableCellProxy#method.get_row_header_cells
//! [`get_column_header_cells`]: TableCellProxy#method.get_column_header_cells
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

use atspi_common::object_ref::ObjectRefOwned;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
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
