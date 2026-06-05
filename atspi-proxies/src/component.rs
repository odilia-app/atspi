//! # `ComponentProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Component`
//! interface.
//!
//! The `Component` interface provides methods to query and manipulate the visual
//! and spatial aspects of a UI component. This includes retrieving its absolute
//! and relative bounding box coordinates ([`get_extents`]), determining its
//! layering order ([`get_layer`]), checking focus state, grabbing keyboard focus,
//! and resolving which accessible element resides at a specific screen point ([`get_accessible_at_point`]).
//!
//! ## Defaults
//!
//! The `Component` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain a `ComponentProxy`
//!
//! There are two idiomatic ways to obtain a `ComponentProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a visual node, you can safely
//! query and convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let component = proxies.component().await?;
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
//! let component = ComponentProxy::builder(&connection)
//!     .destination(service_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`get_extents`]: ComponentProxy#method.get_extents
//! [`get_layer`]: ComponentProxy#method.get_layer
//! [`get_accessible_at_point`]: ComponentProxy#method.get_accessible_at_point
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

use crate::common::{CoordType, Layer, ScrollType};
use atspi_common::object_ref::ObjectRefOwned;

// We don't want the proxy macro to auto-derive
// defaults, so assume_defaults is explicitly
// set to false.
#[zbus::proxy(interface = "org.a11y.atspi.Component", assume_defaults = false)]
pub trait Component {
	/// `Contains` method
	fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<bool>;

	/// `GetAccessibleAtPoint` method
	/// To get an accessible at a point inside a frame of a particular app, you must use `CoordType::Window`
	fn get_accessible_at_point(
		&self,
		x: i32,
		y: i32,
		coord_type: CoordType,
	) -> zbus::Result<ObjectRefOwned>;

	/// `GetAlpha` method
	fn get_alpha(&self) -> zbus::Result<f64>;

	/// `GetExtents` method
	fn get_extents(&self, coord_type: CoordType) -> zbus::Result<(i32, i32, i32, i32)>;

	/// `GetLayer` method
	fn get_layer(&self) -> zbus::Result<Layer>;

	/// `GetMDIZOrder` method
	#[zbus(name = "GetMDIZOrder")]
	fn get_mdiz_order(&self) -> zbus::Result<i16>;

	/// `GetPosition` method
	/// To get the position of a frame of a particular app, you must use `CoordType::Screen`
	fn get_position(&self, coord_type: CoordType) -> zbus::Result<(i32, i32)>;

	/// `GetSize` method
	fn get_size(&self) -> zbus::Result<(i32, i32)>;

	/// `GrabFocus` method
	fn grab_focus(&self) -> zbus::Result<bool>;

	/// `ScrollTo` method
	fn scroll_to(&self, type_: ScrollType) -> zbus::Result<bool>;

	/// `ScrollToPoint` method
	fn scroll_to_point(&self, coord_type: CoordType, x: i32, y: i32) -> zbus::Result<bool>;

	/// `SetExtents` method
	fn set_extents(
		&self,
		x: i32,
		y: i32,
		width: i32,
		height: i32,
		coord_type: CoordType,
	) -> zbus::Result<bool>;

	/// `SetPosition` method
	fn set_position(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<bool>;

	/// `SetSize` method
	fn set_size(&self, width: i32, height: i32) -> zbus::Result<bool>;
}
