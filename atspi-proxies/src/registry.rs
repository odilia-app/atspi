//! Due to these fixed defaults, the proxy can be constructed with zero-configuration
//! using the shorthand `new` constructor.
//!
//! ## [`AccessibilityConnection`][ac] & [`Deref`] Integration
//!
//! If you are using the high-level `AccessibilityConnection` from the `atspi-connection`
//! crate, you rarely need to instantiate `RegistryProxy` manually:
//!
//! 1. **Automatic Deref**: `AccessibilityConnection` implements `Deref` with
//!    `RegistryProxy<'static>` as its target. Any method on `RegistryProxy` (like the
//!    string-based [`register_event`]) can be called directly on an `AccessibilityConnection` instance.
//! 2. **Type-Safe Helpers**: `AccessibilityConnection` also provides type-safe, generic
//!    overloads of [`register_event` on `AccessibilityConnection`][ac_re] and
//!    [`deregister_event` on `AccessibilityConnection`][ac_de] to subscribe to strongly-typed
//!    Rust event structs (rather than raw D-Bus strings).
//!
//! ## How to instantiate the proxy manually
//!
//! If you are not using `AccessibilityConnection` but want to instantiate `RegistryProxy`
//! directly on the **accessibility bus**:
//!
//! ### 1. Shorthand construction using `new` (Recommended)
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::registry::RegistryProxy;
//!
//! # block_on( async {
//! // 1. Open a connection to the dedicated accessibility bus:
//! let a11y_connection = AccessibilityConnection::new().await?;
//! let conn = a11y_connection.connection();
//!
//! // 2. Create the RegistryProxy:
//! let registry = RegistryProxy::new(conn).await?;
//!
//! // 3. Register for global focus changes across the desktop:
//! registry.register_event("object:state-changed:focused").await?;
//! # Ok::<(), atspi_common::AtspiError>(())
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
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::registry::RegistryProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y_connection = AccessibilityConnection::new().await?;
//! let conn = a11y_connection.connection();
//!
//! let registry = RegistryProxy::builder(conn)
//!     .cache_properties(CacheProperties::No) // Caching uitgeschakeld!
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [`register_event`]: RegistryProxy#method.register_event
//! [`deregister_event`]: RegistryProxy#method.deregister_event
//! [`registered_events`]: RegistryProxy#method.registered_events
//! [or]: crate::common::ObjectRef
//! [deref]: https://doc.rust-lang.org/std/ops/trait.Deref.html
//! [ac_re]: crate::connection::AccessibilityConnection::deregister_event
//! [ac_de]: crate::connection::AccessibilityConnection::deregister_event
//! [ac]: crate::connection::AccessibilityConnection

use zbus::names::OwnedBusName;

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(
	interface = "org.a11y.atspi.Registry",
	default_service = "org.a11y.atspi.Registry",
	default_path = "/org/a11y/atspi/registry",
	assume_defaults = false
)]
pub trait Registry {
	/// `DeregisterEvent` method
	fn deregister_event(&self, event: &str) -> zbus::Result<()>;

	/// `GetRegisteredEvents` method
	#[zbus(name = "GetRegisteredEvents")]
	fn registered_events(&self) -> zbus::Result<Vec<(OwnedBusName, String)>>;

	/// `RegisterEvent` method
	fn register_event(&self, event: &str) -> zbus::Result<()>;
}
