//! # `ApplicationProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Application`
//! interface.
//!
//! `Application` is the interface implemented by each accessible application's
//! root object. It provides metadata and runtime information about the application
//! itself (such as its name, version, and the toolkit used to build it).
//!
//! ## Status and Deprecations
//!
//! Several methods and properties of this interface are legacy or considered
//! deprecated, and should be avoided in modern implementations:
//!
//! * [`id`](ApplicationProxy#method.id) & [`set_id`](ApplicationProxy#method.set_id)
//! * [`atspi_version`](ApplicationProxy#method.atspi_version)
//! * [`get_locale`](ApplicationProxy#method.get_locale)
//!
//! Active and supported properties you should rely on:
//!
//! * [`toolkit_name`](ApplicationProxy#method.toolkit_name)
//! * [`version`](ApplicationProxy#method.version)
//!
//! ## Defaults
//!
//! The `Application` interface is always implemented on the **root node** of an
//! application's UI-tree. Because this root object path is fixed across all
//! AT-SPI2 applications, the proxy defines a fixed default path:
//! `/org/a11y/atspi/accessible/root`.
//!
//! ## How to obtain an `ApplicationProxy`
//!
//! There are three idiomatic ways to obtain an `ApplicationProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you have an [`AccessibleProxy`][ap] pointing to the application's root node,
//! you can query its interfaces and convert it safely:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = root_node.proxies().await?;
//! let application = proxies.application().await?;
//! ```
//!
//! ### 2. Manual construction using the `builder` (Fixed Path)
//! Because the object path of the `Application` interface is fixed, you only need
//! to supply the application's unique D-Bus service destination. The builder will
//! automatically use the default path:
//!
//! ```rust,ignore
//! let application = ApplicationProxy::builder(&connection)
//!     .destination(service_name)?
//!     // No path is specified; the default root path is used automatically
//!     .build()
//!     .await?;
//! ```
//!
//! ### 3. Construction using `new`
//! Alternatively, you can instantiate the proxy directly using the short-hand
//! `new` constructor, which requires only the connection and destination:
//!
//! ```rust,ignore
//! let application = ApplicationProxy::new(&connection, service_name).await?;
//! ```
//!
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy

#[zbus::proxy(
	interface = "org.a11y.atspi.Application",
	default_path = "/org/a11y/atspi/accessible/root",
	assume_defaults = false
)]
pub trait Application {
	/// Method to retrieve the application's locale.
	///
	/// ## Deprecation
	///
	/// This method is likely to be removed in the future.
	///
	/// There is no need to call this method because there is also
	/// [`locale`] which offers the same functionality
	/// at the accessible object level.
	///
	/// See also: [Orca issues: "Plans for per-object locale?"](<https://gitlab.gnome.org/GNOME/orca/-/issues/260>)
	///
	/// member: `GetLocale`, type: method
	///
	/// [`locale`]: crate::accessible::AccessibleProxy#method.locale
	fn get_locale(&self, lctype: u32) -> zbus::Result<String>;

	/// retrieves AT-SPI2 version.
	///
	/// Applications are advised to return "2.1" here, thus that is what
	/// users should expect.
	///
	/// This was intended to be the version of the atspi interfaces
	/// that the application supports, but atspi will probably move to
	/// using versioned interface names instead.
	///
	/// member: `AtspiVersion`, type: property
	#[zbus(property)]
	fn atspi_version(&self) -> zbus::Result<String>;

	/// Retrieve numerical id of the application.
	///
	/// The 'id' is set an arbitrary numerical id when
	/// an application registers with the registry.
	///
	/// When a freshly-started application uses the
	/// [`org.a11y.atspi.Socket`]'s [`embed`] method to
	/// register with the accessibility registry, the
	/// registry will set a numerical id on the application.
	///
	/// ## status
	///
	/// The property has fallen in disuse.
	///
	/// As per [AT-SPI2-CORE issue #82](<https://gitlab.gnome.org/GNOME/at-spi2-core/-/issues/82>)
	/// it may turn out that this id is not actually used subsequently.
	/// This is a remnant of the time when registryd actually had to
	/// make up identifiers for each application.
	/// With `DBus`, however,	it is the bus that assigns unique names to applications that
	/// connect to it.
	///
	/// Applications or toolkits can remember the `Id` passed when the accessibility
	/// registry sets this property, and return it back when the property is read.
	///
	/// member: `Id`, type: property
	///
	/// [`embed`]: crate::socket::SocketProxy#method.embed
	/// [`org.a11y.atspi.Socket`]: crate::socket::SocketProxy
	#[zbus(property)]
	fn id(&self) -> zbus::Result<i32>;

	/// Set ID of the application.
	///
	/// This method is used by the accessibility registry to set the
	/// application's id.
	///
	/// ## status
	///
	/// The property has fallen in disuse.
	///
	/// See [`id`] for details.
	///
	/// member: `Id`, type: property
	///
	/// [`id`]: crate::application::ApplicationProxy#method.id
	#[zbus(property)]
	fn set_id(&self, value: i32) -> zbus::Result<()>;

	/// Retrieves the name of the toolkit used to implement the application's
	/// user interface.
	///
	/// member: `ToolkitName`, type: property
	#[zbus(property)]
	fn toolkit_name(&self) -> zbus::Result<String>;

	/// Returns the version of the toolkit used to implement the
	/// application's user interface.
	///
	/// member: `Version`, type: property
	#[zbus(property)]
	fn version(&self) -> zbus::Result<String>;

	/// Method to obtain the unix socket address.
	/// The unix socket can be used to setup a connection, to perform peer-to-peer (P2P) method calls.
	///
	/// Known implementors include `Gtk3` and `Firefox`.
	fn get_application_bus_address(&self) -> zbus::Result<String>;
}
