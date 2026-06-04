//! # `ImageProxy`
//!
//! `org.a11y.atspi.Image` interface provides access to image data.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Image" may be implemented for individual nodes
//! in the application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this `DocumentProxy` is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.

use crate::CoordType;

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
