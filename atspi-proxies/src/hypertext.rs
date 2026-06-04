//! # `HypertextProxy`
//!
//! `org.a11y.atspi.Hypertext` provides access to hypertext information.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Hypertext" may be implemented for individual nodes
//! in the application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this `HypertextProxy` is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.

use atspi_common::object_ref::ObjectRefOwned;

#[zbus::proxy(interface = "org.a11y.atspi.Hypertext", assume_defaults = false)]
pub trait Hypertext {
	/// `GetLink` method
	fn get_link(&self, link_index: i32) -> zbus::Result<ObjectRefOwned>;

	/// `GetLinkIndex` method
	fn get_link_index(&self, character_index: i32) -> zbus::Result<i32>;

	/// `GetNLinks` method
	#[zbus(name = "GetNLinks")]
	fn get_n_links(&self) -> zbus::Result<i32>;
}
