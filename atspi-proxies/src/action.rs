//! # [`ActionProxy`][ActionProxy]
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Action`
//! interface.
//!
//! The `Action` interface allows exploring and invoking the actions of a
//! user-actionable UI component.
//!
//! For example, a button may expose a "click" action - a popup menu may
//! expose an "open" action.
//!
//! Components which are not "passive" providers of UI information should
//! implement this interface, unless there is a more specialized interface for
//! interaction like [`org.a11y.atspi.Text`][TextProxy] or [`org.a11y.atspi.Value`][ValueProxy].
//!  
//! [ActionProxy]: crate::action::ActionProxy
//! [TextProxy]: crate::text::TextProxy
//! [ValueProxy]: crate::value::ValueProxy

use crate::atspi_proxy;

/// A handle for a remote object implementing the `org.a11y.atspi.Action`
/// interface.
///
/// The `Action` interface allows exploring and invoking the actions of a
/// user-actionable UI component.
///
/// For example, a button may expose a "click" action - a popup menu may
/// expose an "open" action.
///
/// Components which are not "passive" providers of UI information should
/// implement this interface, unless there is a more specialized interface for
/// interaction like [`org.a11y.atspi.Text`][TextProxy] or [`org.a11y.atspi.Value`][ValueProxy].
///  
/// [TextProxy]: crate::text::TextProxy
/// [ValueProxy]: crate::value::ValueProxy
#[atspi_proxy(interface = "org.a11y.atspi.Action", assume_defaults = true)]
trait Action {
	/// Performs the specified action on the object.
	///
	/// Returns: Ok(true) on success, Ok(false) otherwise.
	///
	/// # Arguments
	///
	/// * `index` - The index of the action to perform.
	fn do_action(&self, index: i32) -> zbus::Result<bool>;

	/// Returns an array of (localized-) name, (localized-)
	/// description, keybinding) for the actions that an object
	/// supports.
	///
	/// See [`get_keybinding`] method for a description of that
	/// field's syntax.
	///
	/// This is equivalent to using the methods [`get_localized_name`],
	/// [`get_description`] and	[`get_keybinding`] for each action,
	/// but with a single call and thus less DBus traffic.
	///
	///	By convention, if there is more than one action available,
	/// the first one is considered the "default" action of the object.
	fn get_actions(&self) -> zbus::Result<Vec<(String, String, String)>>;

	/// GetDescription method
	fn get_description(&self, index: i32) -> zbus::Result<String>;

	/// GetKeyBinding method
	fn get_key_binding(&self, index: i32) -> zbus::Result<String>;

	/// GetLocalizedName method
	fn get_localized_name(&self, index: i32) -> zbus::Result<String>;

	/// GetName method
	fn get_name(&self, index: i32) -> zbus::Result<String>;

	/// NActions property
	#[dbus_proxy(property)]
	fn nactions(&self) -> zbus::Result<i32>;
}
