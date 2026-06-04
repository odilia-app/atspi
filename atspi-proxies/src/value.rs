//! # `ValueProxy`
//!
//! `org.a11y.atspi.Value` provides methods to interact with UI elements that
//! represent a value.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Value" may be implemented for individual nodes
//! in the application's UI-tree.
//!
//! Service and path are either provided by the builder or inherited from the
//! [`zbus::Proxy`] this `TableProxy` is derived from.
//!
//! No default service or default path makes sense for this proxy, thus
//! the macro is instructed explicitly not to generate the defaults.

#[zbus::proxy(interface = "org.a11y.atspi.Value", assume_defaults = false)]
pub trait Value {
	/// `CurrentValue` property
	#[zbus(property)]
	fn current_value(&self) -> zbus::Result<f64>;

	/// Set `CurrentValue` property
	#[zbus(property)]
	fn set_current_value(&self, value: f64) -> zbus::Result<()>;

	/// `MaximumValue` property
	#[zbus(property)]
	fn maximum_value(&self) -> zbus::Result<f64>;

	/// `MinimumIncrement` property
	#[zbus(property)]
	fn minimum_increment(&self) -> zbus::Result<f64>;

	/// `MinimumValue` property
	#[zbus(property)]
	fn minimum_value(&self) -> zbus::Result<f64>;

	/// `Text` property
	#[zbus(property)]
	fn text(&self) -> zbus::Result<String>;
}
