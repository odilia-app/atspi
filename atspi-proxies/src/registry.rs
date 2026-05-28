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

use atspi_common::object_ref::ObjectRefOwned;
use zbus::names::OwnedBusName;

use crate::accessible::{AccessibleProxy, ObjectRefExt};
use crate::AtspiError;

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

impl RegistryProxy<'_> {
	/// Find application roots whose accessible `Name` equals `name`.
	///
	/// Walks the registry's children and returns every match. More than one
	/// connection may publish a root with the same `Name`, so multiple
	/// matches are possible; the `Vec` is empty when nothing matches.
	///
	/// Per-child failures (process gone, denied property read, transient
	/// `DBus` error) are skipped rather than surfaced.
	///
	/// For non-exact matching, see [`find_by_name_with`].
	///
	/// [`find_by_name_with`]: RegistryProxy::find_by_name_with
	///
	/// # Errors
	///
	/// Errors if the registry view cannot be built or [`get_children`] fails.
	///
	/// [`get_children`]: crate::accessible::AccessibleProxy#method.get_children
	pub async fn find_by_name(&self, name: &str) -> Result<Vec<ObjectRefOwned>, AtspiError> {
		self.find_by_name_with(|candidate| candidate == name).await
	}

	/// Find application roots whose accessible `Name` is accepted by `predicate`.
	///
	/// User-defined-match variant of [`find_by_name`]; use for case-insensitive,
	/// prefix, or regex matching. The predicate is invoked once per registry
	/// child; children that fail to materialize are skipped without calling it.
	///
	/// [`find_by_name`]: RegistryProxy::find_by_name
	///
	/// # Errors
	///
	/// Errors if the registry view cannot be built or [`get_children`] fails.
	///
	/// [`get_children`]: crate::accessible::AccessibleProxy#method.get_children
	pub async fn find_by_name_with<F>(
		&self,
		mut predicate: F,
	) -> Result<Vec<ObjectRefOwned>, AtspiError>
	where
		F: FnMut(&str) -> bool,
	{
		let inner = self.inner();
		let conn = inner.connection();

		// `get_children` lives on `Accessible`, not `Registry`, and application
		// roots are children of the root accessible. Build an `AccessibleProxy`
		// view of the root accessible (the registry's well-known name plus the
		// standard root path), with caching disabled because the registry's
		// `Properties` impl is incomplete. Mirrors
		// `AccessibilityConnection::root_accessible_on_registry`.
		let root_accessible = AccessibleProxy::builder(conn)
			.destination(inner.destination().clone())?
			.path(atspi_common::ACCESSIBLE_ROOT_PATH)?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await?;

		let children = root_accessible.get_children().await?;

		let mut matches = Vec::new();
		for child in children {
			// Skip per-child failures (process gone, denied read, `DBus` hiccup);
			// the walk continues.
			let Ok(child_proxy) = child.as_accessible_proxy(conn).await else {
				continue;
			};
			let Ok(child_name) = child_proxy.name().await else {
				continue;
			};

			if predicate(child_name.as_str()) {
				matches.push(child);
			}
		}

		Ok(matches)
	}
}
