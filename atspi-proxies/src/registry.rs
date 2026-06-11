//! # `RegistryProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Registry`
//! interface.
//!
//! The `Registry` is the central daemon in AT-SPI2. It acts as the coordinator and
//! routing hub, managing event subscriptions and dispatching events between
//! application nodes and assistive technologies.
//!
//! ## D-Bus Addressing
//!
//! For Assistive Technology (AT) clients—such as screen readers—this is the primary
//! entry point used to register for global desktop events (such as focus changes,
//! window activations, or caret movements) via [`register_event`].
//!
//! The Registry is a central, static service running on the accessibility bus,
//! residing at a fixed and well-known address:
//!
//! * **Service Destination**: `org.a11y.atspi.Registry`
//! * **Object Path**: `/org/a11y/atspi/registry`
//!
//! Because these addressing details are standardized, the proxy can be instantiated
//! directly on the accessibility bus using its shorthand `new` constructor.
//!
//! ## [`AccessibilityConnection`][ac] & [`Deref`][deref] Integration
//!
//! When utilizing the high-level `AccessibilityConnection` from the `atspi-connection`
//! crate, manual instantiation of `RegistryProxy` is rarely required:
//!
//! 1. **Automatic Deref**: `AccessibilityConnection` implements `Deref` with
//!    `RegistryProxy<'static>` as its target. Any method on `RegistryProxy` (such as the
//!    string-based [`register_event`]) can be called directly on an `AccessibilityConnection` instance.
//! 2. **Type-Safe Helpers**: `AccessibilityConnection` also provides type-safe, generic
//!    overloads of [`register_event` on `AccessibilityConnection`][ac_re] and
//!    [`deregister_event` on `AccessibilityConnection`][ac_de] to subscribe to strongly-typed
//!    Rust event structs (rather than raw D-Bus strings).
//!
//! ## How to instantiate the proxy manually
//!
//! When not using `AccessibilityConnection`, `RegistryProxy` can be instantiated
//! directly on the accessibility bus:
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
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! ## [`AccessibilityConnection`][ac] & [`Deref`][deref] Integration
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
//! [deref]: std::ops::Deref
//! [ac_re]: ../../atspi-connection/struct.AccessibilityConnection.html#method.register_event
//! [ac_de]: ../../atspi-connection/struct.AccessibilityConnection.html#method.deregister_event
//! [ac]: ../../atspi-connection/struct.AccessibilityConnection.html

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
