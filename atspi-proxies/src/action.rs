//! # `ActionProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Action`
//! interface.
//!
//! The `Action` interface allows exploring and invoking the actions of a
//! user-actionable UI component. For example, a button may expose a "click"
//! action, while a popup menu may expose an "open" action.
//!
//! Active components that are not "passive" providers of UI information should
//! implement this interface, unless there is a more specialized interface for
//! interaction (such as [`TextProxy`][tp] or [`ValueProxy`][vp]).
//!
//! ## D-Bus Addressing
//!
//! Since this interface is implemented dynamically on individual nodes within an
//! application's UI-tree, its D-Bus addressing (the unique bus name and object path)
//! varies per node. There is no static, well-known service destination or object path
//! applicable; address details must be resolved dynamically at runtime.
//!
//! ## How to obtain an `ActionProxy`
//!
//! There are two idiomatic ways to obtain an `ActionProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] for an actionable node, you can
//! safely query and convert it using the [`ProxyExt`][pe] trait:
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
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked("1:1000", "/org/a11y/atspi/accessible/root");
//! let accessible_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! // Get the associated interface proxies
//! let proxies = accessible_node.proxies().await?;
//! let action = proxies.action().await?;
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
//! use atspi_proxies::action::ActionProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let action = ActionProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [tp]: crate::text::TextProxy
//! [vp]: crate::value::ValueProxy

use atspi_common::Action;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>

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
#[zbus::proxy(interface = "org.a11y.atspi.Action", assume_defaults = false)]
pub trait Action {
	/// Performs the specified action on the object.
	///
	/// Returns: Ok(true) on success, Ok(false) otherwise.
	///
	/// # Arguments
	///
	/// * `index` - The index of the action to perform.
	fn do_action(&self, index: i32) -> zbus::Result<bool>;

	/// Returns an array of localized name, localized
	/// description, keybinding for the actions that an object
	/// supports.
	///
	/// See [`get_key_binding`] method for a description of that
	/// field's syntax.
	///
	/// This is equivalent to using the methods [`get_localized_name`],
	/// [`get_description`] and	[`get_key_binding`] for each action,
	/// but with a single call and thus less `DBus` traffic.
	///
	///	By convention, if there is more than one action available,
	/// the first one is considered the "default" action of the object.
	///
	/// [`get_key_binding`]: ActionProxy#method.get_key_binding
	/// [`get_localized_name`]: ActionProxy#method.get_localized_name
	/// [`get_description`]: ActionProxy#method.get_description
	fn get_actions(&self) -> zbus::Result<Vec<Action>>;

	/// Returns the localized description for the action at the specified
	/// index, starting at zero.
	///
	/// For	example, a screen reader will read out this description when
	/// the user asks for extra detail on an action.
	/// For example, "Clicks the button" for the "click" action of a button.
	fn get_description(&self, index: i32) -> zbus::Result<String>;

	/// Returns the keybinding for the action, specified by a
	/// zero-based index.
	///
	/// Gets the keybinding which can be used to invoke this action,
	/// if one exists.
	///
	/// The string returned should contain localized, human-readable,
	/// key sequences as they would appear when displayed on screen.
	/// It must be in the format "mnemonic;sequence;shortcut".
	///
	/// - The mnemonic key activates the object if it is presently
	/// enabled on screen.
	/// This typically corresponds to the underlined letter within
	/// the widget. Example: "n" in a traditional "Ṉew..." menu
	/// item or the "a" in "Apply" for a button.
	///
	/// - The sequence is the full list of keys which invoke the action
	/// even if the relevant element is not currently shown on screen.
	/// For instance, for a menu item the sequence is the keybindings
	/// used to open the parent menus before invoking.
	///
	/// The sequence string is colon-delimited. Example: "Alt+F:N" in a
	/// traditional "Ṉew..." menu item.
	///
	/// - The shortcut, if it exists, will invoke the same action without
	/// showing the component or its enclosing menus or dialogs.
	/// Example: "Ctrl+N" in a traditional "Ṉew..." menu item.
	/// The shortcut string is colon-delimited. Example: "Ctrl+N" in a
	/// traditional "Ṉew..." menu item.
	///
	/// Example: For a traditional "Ṉew..." menu item, the expected return
	/// value would be: "N;Alt+F:N;Ctrl+N" for the English locale and
	/// "N;Alt+D:N;Strg+N" for the German locale.
	/// If, hypothetically, this menu item lacked a mnemonic, it would be
	/// represented by ";;Ctrl+N" and ";;Strg+N" respectively.
	///
	/// If there is no key binding for this action, "" is returned.
	fn get_key_binding(&self, index: i32) -> zbus::Result<String>;

	/// Returns a short, localized name for the action at the specified by a
	/// zero-based index.
	///
	/// This is	what screen readers will read out during normal navigation.
	/// For example, "Click" for a button.
	fn get_localized_name(&self, index: i32) -> zbus::Result<String>;

	/// Returns a machine-readable name for the action at the specified,
	/// zero-based index.
	fn get_name(&self, index: i32) -> zbus::Result<String>;

	/// Returns the number of available actions.
	///
	/// By convention, if there is more than one action available,
	/// the first one is considered the "default" action of the object.
	#[zbus(property, name = "NActions")]
	fn n_actions(&self) -> zbus::Result<i32>;
}
