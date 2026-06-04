//! # `HyperlinkProxy`
//!
//! `org.a11y.atspi.Hyperlink` provides access to hyperlink information.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Hyperlink" may be implemented for individual nodes
//! in the application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this `HyperlinkProxy` is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.

use atspi_common::object_ref::ObjectRefOwned;

#[zbus::proxy(interface = "org.a11y.atspi.Hyperlink", assume_defaults = false)]
pub trait Hyperlink {
	/// `GetObject` method
	fn get_object(&self, i: i32) -> zbus::Result<ObjectRefOwned>;

	/// `GetURI` method
	#[zbus(name = "GetURI")]
	fn get_uri(&self, i: i32) -> zbus::Result<String>;

	/// `IsValid` method
	fn is_valid(&self) -> zbus::Result<bool>;

	/// `EndIndex` property
	#[zbus(property)]
	fn end_index(&self) -> zbus::Result<i32>;

	/// `NAnchors` property
	#[zbus(property, name = "NAnchors")]
	fn n_anchors(&self) -> zbus::Result<i16>;

	/// `StartIndex` property
	#[zbus(property)]
	fn start_index(&self) -> zbus::Result<i32>;
}
