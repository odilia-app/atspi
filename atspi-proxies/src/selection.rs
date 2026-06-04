//! # `SelectionProxy`
//!
//! `org.a11y.atspi.Selection` provides access to the selections
//! and selected objects in the UI-tree.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Selection" may be implemented for individual nodes
//! in the application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this [`SelectionProxy`] is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.
//!
use atspi_common::object_ref::ObjectRefOwned;

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
