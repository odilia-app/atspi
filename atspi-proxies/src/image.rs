//! # `ImageProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Image`
//! interface.
//!
//! The `Image` interface provides access to metadata and spatial bounds for visual
//! elements such as icons, photos, charts, and custom drawings.
//!
//! Primarily, it allows screen readers and other AT clients to retrieve:
//!
//! * **Alternative Text**: A textual description of what the image represents ([`image_description`]).
//! * **Spatial Bounds**: The exact position and size of the image on the screen or window ([`get_image_extents`]).
//!
//! ## Defaults
//!
//! The `Image` interface can be implemented on any individual node within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path is applicable for this proxy.
//!
//! ## How to obtain an `ImageProxy`
//!
//! There are two idiomatic ways to obtain an `ImageProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for an image node, you can safely
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
//! // Establish an `AccessibleProxy` for the image node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let accessible_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! let proxies = accessible_node.proxies().await?;
//! let image = proxies.image().await?;
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
//! use atspi_proxies::image::ImageProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let image = ImageProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No) // Caching uitgeschakeld!
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [`image_description`]: ImageProxy#method.image_description
//! [`get_image_extents`]: ImageProxy#method.get_image_extents
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

use crate::CoordType;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(interface = "org.a11y.atspi.Image", assume_defaults = false)]
pub trait Image {
	/// `GetImageExtents` method
	fn get_image_extents(&self, coord_type: CoordType) -> zbus::Result<(i32, i32, i32, i32)>;

	/// `GetImagePosition` method
	fn get_image_position(&self, coord_type: CoordType) -> zbus::Result<(i32, i32)>;

	/// `GetImageSize` method
	fn get_image_size(&self) -> zbus::Result<(i32, i32)>;

	/// `ImageDescription` property
	#[zbus(property)]
	fn image_description(&self) -> zbus::Result<String>;

	/// `ImageLocale` property
	#[zbus(property)]
	fn image_locale(&self) -> zbus::Result<String>;
}
