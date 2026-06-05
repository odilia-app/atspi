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
//! The `Image` interface is implemented on individual, variable nodes within the
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
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let image = proxies.image().await?;
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
//! let image = ImageProxy::builder(&connection)
//!     .destination(service_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`image_description`]: ImageProxy#method.image_description
//! [`get_image_extents`]: ImageProxy#method.get_image_extents
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

use crate::CoordType;

// We explicitly disable the `assume_defaults` option to avoid generating default service/path methods.
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
