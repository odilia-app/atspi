//! # `SelectionProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Selection`
//! interface.
//!
//! The `Selection` interface is implemented by UI containers that support selecting
//! their child elements (such as list boxes, tree views, tables, or tab lists).
//!
//! It provides methods to inspect the current selection state (such as [`n_selected_children`]
//! or [`get_selected_child`]), and programmatically manipulate the selection (such as
//! [`select_child`], [`deselect_child`], or [`clear_selection`]).
//!
//! ## Defaults
//!
//! The `Selection` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain a `SelectionProxy`
//!
//! There are two idiomatic ways to obtain a `SelectionProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a container node, you can safely
//! query and convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let selection = proxies.selection().await?;
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
//! let selection = SelectionProxy::builder(&connection)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`n_selected_children`]: SelectionProxy#method.n_selected_children
//! [`get_selected_child`]: SelectionProxy#method.get_selected_child
//! [`select_child`]: SelectionProxy#method.select_child
//! [`deselect_child`]: SelectionProxy#method.deselect_child
//! [`clear_selection`]: SelectionProxy#method.clear_selection
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

use atspi_common::object_ref::ObjectRefOwned;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(interface = "org.a11y.atspi.Selection", assume_defaults = false)]
pub trait Selection {
	/// `ClearSelection` method
	fn clear_selection(&self) -> zbus::Result<bool>;

	/// `DeselectChild` method
	fn deselect_child(&self, child_index: i32) -> zbus::Result<bool>;

	/// `DeselectSelectedChild` method
	fn deselect_selected_child(&self, selected_child_index: i32) -> zbus::Result<bool>;

	/// `GetSelectedChild` method
	fn get_selected_child(&self, selected_child_index: i32) -> zbus::Result<ObjectRefOwned>;

	/// `IsChildSelected` method
	fn is_child_selected(&self, child_index: i32) -> zbus::Result<bool>;

	/// `SelectAll` method
	fn select_all(&self) -> zbus::Result<bool>;

	/// `SelectChild` method
	fn select_child(&self, child_index: i32) -> zbus::Result<bool>;

	/// `NSelectedChildren` property
	#[zbus(property, name = "NSelectedChildren")]
	fn n_selected_children(&self) -> zbus::Result<i32>;
}
