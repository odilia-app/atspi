//! # `BusProxy` & `StatusProxy`
//!
//! Handles for the remote objects implementing the `org.a11y.Bus` and
//! `org.a11y.Status` interfaces on the D-Bus session bus.
//!
//! Together, these interfaces are exposed by a small, central broker daemon
//! (`at-spi-bus-launcher`) which acts as the entry point for AT-SPI2 accessibility
//! on the user's session.
//!
//! * **`org.a11y.Bus`**: Offers a single method ([`get_address`]) to obtain the
//!   address of the dedicated, private accessibility bus where all application UI
//!   nodes actually live.
//! * **`org.a11y.Status`**: Provides properties to check and modify the global
//!   accessibility status, such as whether accessibility ([`is_enabled`]) or the
//!   screen reader ([`screen_reader_enabled`]) are currently active.
//!
//! ## D-Bus Addressing
//!
//! Unlike dynamic UI-tree proxies, both interfaces are hosted by a single,
//! static daemon on the D-Bus session bus. They reside at fixed, well-known
//! addresses:
//!
//! * **Service Destination**: `org.a11y.Bus`
//! * **Object Path**: `/org/a11y/bus`
//!
//! Because these locations are standardized, both proxies can be instantiated
//! directly using the shorthand `new` constructor.
//!
//! ## How to instantiate the proxies
//!
//! Because these are central services, you never obtain them from UI-nodes or
//! [`ObjectRef`][or]s. Instead, you instantiate them directly on the **session bus**:
//!
//! ### 1. Shorthand construction using `new` (Recommended)
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_proxies::bus::{BusProxy, StatusProxy};
//!
//! # block_on( async {
//! let session_bus = zbus::Connection::session().await?;
//!
//! // Obtain the private accessibility bus address:
//! let bus_proxy = BusProxy::new(&session_bus).await?;
//! let _a11y_address = bus_proxy.get_address().await?;
//!
//! // Check or toggle global accessibility status:
//! let status_proxy = StatusProxy::new(&session_bus).await?;
//! if status_proxy.is_enabled().await? {
//!     println!("AT-SPI2 accessibility is active.");
//! }
//! # Ok::<(), zbus::Error>(())
//! # });
//! ```
//!
//! ### 2. Construction using the `builder`
//! If you need custom proxy configuration, you can use the builder. You do not
//! need to specify the path or destination, as the defaults are automatically
//! filled in:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_proxies::bus::BusProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let session_bus = zbus::Connection::session().await?;
//!
//! let bus_proxy = BusProxy::builder(&session_bus)
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), zbus::Error>(())
//! # });
//! ```
//!
//! ## High-level Helpers
//!
//! If you only need to read or set the global session accessibility status, you
//! do not need to manually construct `StatusProxy` at all. The `atspi-connection`
//! crate offers two convenient, high-level helper functions that handle the
//! session bus connection and status check/toggle on your behalf:
//!
//! * **[`read_session_accessibility`][rsa]**: Directly queries whether AT-SPI2
//!   accessibility is enabled on the session bus.
//! * **[`set_session_accessibility`][ssa]**: Enables or disables AT-SPI2 accessibility
//!   on the session bus.
//!
//! [`get_address`]: BusProxy#method.get_address
//! [`is_enabled`]: StatusProxy#method.is_enabled
//! [`screen_reader_enabled`]: StatusProxy#method.screen_reader_enabled
//! [or]: crate::common::ObjectRef
//! [rsa]: https://docs.rs/atspi-connection/latest/atspi_connection/fn.read_session_accessibility.html
//! [ssa]: https://docs.rs/atspi-connection/latest/atspi_connection/fn.set_session_accessibility.html

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(
	interface = "org.a11y.Status",
	default_service = "org.a11y.Bus",
	default_path = "/org/a11y/bus",
	assume_defaults = false
)]
pub trait Status {
	/// `IsEnabled` property
	#[zbus(property)]
	fn is_enabled(&self) -> zbus::Result<bool>;
	#[zbus(property)]
	fn set_is_enabled(&self, value: bool) -> zbus::Result<()>;

	/// `ScreenReaderEnabled` property
	#[zbus(property)]
	fn screen_reader_enabled(&self) -> zbus::Result<bool>;
	#[zbus(property)]
	fn set_screen_reader_enabled(&self, value: bool) -> zbus::Result<()>;
}

// See note on `assume_defaults` on `org.a11y.Status`
#[zbus::proxy(
	interface = "org.a11y.Bus",
	default_service = "org.a11y.Bus",
	default_path = "/org/a11y/bus",
	assume_defaults = false
)]
pub trait Bus {
	/// `GetAddress` method
	fn get_address(&self) -> zbus::Result<String>;
}
