//! # `ValueProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Value`
//! interface.
//!
//! The `Value` interface provides properties to interact with UI elements that represent
//! a numeric value, a range, or a bounded scale (such as sliders, scrollbars, volume
//! controls, and progress bars). It allows querying the minimum ([`minimum_value`]) and
//! maximum ([`maximum_value`]) limits, checking the smallest allowed step size
//! ([`minimum_increment`]), and getting or setting the current numeric state ([`current_value`], [`set_current_value`]).
//!
//! ## Defaults
//!
//! The `Value` interface is implemented on individual, variable nodes within the
//! application's UI-tree. As a consequence, the object path varies per node and
//! no default path or service is applicable for this proxy.
//!
//! ## How to obtain a `ValueProxy`
//!
//! There are two idiomatic ways to obtain a `ValueProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] pointing to a value-representing node,
//! you can safely query and convert it using the [`ProxyExt`][pe] trait:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = accessible_node.proxies().await?;
//! let value = proxies.value().await?;
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
//! let value = ValueProxy::builder(&connection)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .build()
//!     .await?;
//! ```
//!
//! [`current_value`]: ValueProxy#method.current_value
//! [`set_current_value`]: ValueProxy#method.set_current_value
//! [`minimum_value`]: ValueProxy#method.minimum_value
//! [`maximum_value`]: ValueProxy#method.maximum_value
//! [`minimum_increment`]: ValueProxy#method.minimum_increment
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
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
