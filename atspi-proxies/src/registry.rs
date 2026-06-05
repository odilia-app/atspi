//! # `RegistryProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Registry`
//! interface.
//!
//! The `Registry` interface is the event-driven backbone of the entire AT-SPI2
//! ecosystem. Running centrally on the dedicated **accessibility bus**, its primary
//! responsibility is to coordinate and dispatch global accessibility events from
//! applications to assistive technologies (such as screen readers).
//!
//! Using the `Registry`, clients can:
//!
//! * **Register for Events**: Subscribe globally to specific event streams (such as
//!   `object:state-changed:focused` or `document:load-complete`) using [`register_event`].
//! * **Deregister**: Unsubscribe when those events are no longer needed using [`deregister_event`].
//! * **Query**: Inspect which clients have subscribed to which events via [`registered_events`].
//!
//! Once registered, the central registry daemon will ensure that whenever an application
//! triggers the matching event, a D-Bus signal is routed to your client.
//!
//! ## 💡 Why register events here? (Why not just use D-Bus Match Rules?)
//!
//! While the standard D-Bus broker already has "Match Rules" to filter and route signals,
//! AT-SPI2 requires an additional registry layer for **performance and event silencing**:
//!
//! 1. **Zero-Overhead when Idle**: To save CPU and memory, applications (servers) on the desktop
//!    normally **do not emit any AT-SPI2 signals** at all. The accessibility layer is virtually silent.
//! 2. **On-Demand Activation**: When you call [`register_event`], the `Registry` daemon not only
//!    sets up D-Bus routing, but also notifies all applications on the bus that a client is now
//!    interested in that event.
//! 3. **Dynamic Signal Generation**: Only *after* this registration do applications start spending CPU
//!    cycles to serialize and transmit those high-frequency events (like caret movements or focus changes)
//!    over the D-Bus socket.
//!
//! This cooperative pub-sub architecture ensures the desktop remains fast and responsive when
//! no assistive technologies are active.
//!
//! ## Defaults
//!
//! Because the `Registry` is a single, central service hosting the accessibility bus,
//! it has a fixed well-known D-Bus address:
//!
//! * **Service Destination**: `org.a11y.atspi.Registry`
//! * **Object Path**: `/org/a11y/atspi/registry`
//!
//! Due to these fixed defaults, the proxy can be constructed with zero-configuration
//! using the shorthand `new` constructor.
//!
//! ## How to instantiate the proxy
//!
//! Because the `Registry` is a central service, you never obtain it from UI-nodes or
//! [`ObjectRef`][or]s. Instead, you instantiate it directly on the **accessibility bus**:
//!
//! ### 1. Shorthand construction using `new` (Recommended)
//!
//! ```rust,ignore
//! // 1. Open a connection to the dedicated accessibility bus:
//! let a11y_connection = atspi::AccessibilityConnection::open().await?;
//!
//! // 2. Create the RegistryProxy:
//! let registry = RegistryProxy::new(a11y_connection.connection()).await?;
//!
//! // 3. Register for global focus changes across the desktop:
//! registry.register_event("object:state-changed:focused").await?;
//! ```
//!
//! ### 2. Construction using the `builder`
//! If you need custom proxy configuration, you can use the builder. You do not
//! need to specify the path or destination, as the defaults are automatically
//! filled in:
//!
//! ```rust,ignore
//! let registry = RegistryProxy::builder(a11y_connection.connection())
//!     .cache_properties(zbus::proxy::CacheProperties::No)
//!     .build()
//!     .await?;
//! ```
//!
//! [`register_event`]: RegistryProxy#method.register_event
//! [`deregister_event`]: RegistryProxy#method.deregister_event
//! [`registered_events`]: RegistryProxy#method.registered_events
//! [or]: atspi_common::ObjectRef

use zbus::names::OwnedBusName;

// We explicitly disable the `assume_defaults` option to avoid generating default service/path methods.
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
