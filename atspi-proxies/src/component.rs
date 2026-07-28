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
//! ## D-Bus Addressing
//!
//! Since this interface is implemented dynamically on individual nodes within an
//! application's UI-tree, its D-Bus addressing (the unique bus name and object path)
//! varies per node. There is no static, well-known service destination or object path
//! applicable; address details must be resolved dynamically at runtime.
//!
//! ## How to obtain a `ComponentProxy`
//!
//! There are two idiomatic ways to obtain a `ComponentProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for a visual node, you can safely
//! query and convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::proxy_ext::ProxyExt;
//! use atspi_proxies::accessible::ObjectRefExt;
//! use atspi_common::ObjectRefOwned;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! // Establish an `AccessibleProxy` for the node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let accessible_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! // Get the associated interface proxies
//! let proxies = accessible_node.proxies().await?;
//! let component = proxies.component().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! All proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting any P2P configuration if applicable.
//!
//! ### 2. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::component::ComponentProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let component = ComponentProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [`get_extents`]: ComponentProxy#method.get_extents
//! [`get_layer`]: ComponentProxy#method.get_layer
//! [`get_accessible_at_point`]: ComponentProxy#method.get_accessible_at_point
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

use crate::common::{CoordType, Layer, ScrollType};
use atspi_common::object_ref::ObjectRefOwned;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
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
