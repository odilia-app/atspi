//! # `CacheProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Cache`
//! interface.
//!
//! The `Cache` interface may be a valuable component for AT-SPI2 client performance.
//! Instead of making expensive, synchronous D-Bus method calls to query basic
//! properties on individual UI nodes, clients (such as screen readers) can
//! maintain a local, high-performance representation of the application's UI-tree.
//!
//! To keep this local cache in sync, clients combine:
//!
//! 1. An initial population using [`get_items`] (or [`get_legacy_items`]) to fetch
//!    all active `CacheItem`s.
//! 2. Continuous listening for [`AddAccessibleEvent`][add] and
//!    [`RemoveAccessibleEvent`][remove] to dynamically update the local cache
//!    when UI elements appear or disappear.
//!
//! ## D-Bus Addressing
//!
//! The `Cache` interface is implemented on a single, central object within each
//! application's UI-tree. Because this object path is consistent across all AT-SPI2
//! applications, it resides at a standardized, well-known path:
//!
//! * **Object Path**: `/org/a11y/atspi/cache`
//!
//! ## How to obtain a `CacheProxy`
//!
//! There are three idiomatic ways to obtain a `CacheProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you have an [`AccessibleProxy`][ap] pointing to the application's root node,
//! you can query its interfaces and convert it safely:
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
//! // Establish an `AccessibleProxy` pointing to the application's root node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let root_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! // Convert to `CacheProxy` safely
//! let proxies = root_node.proxies().await?;
//! let cache = proxies.cache().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! All proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting any P2P configuration if applicable.
//!
//! ### 2. Manual construction using the `builder` (Fixed Path)
//! Because the object path of the `Cache` interface is fixed, you only need
//! to supply the application's unique D-Bus service destination. The builder will
//! automatically use the default path:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::cache::CacheProxy;
//! use zbus::proxy::CacheProperties;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//!
//! let cache = CacheProxy::builder(&conn)
//!     .destination(bus_name)?
//!     // No path is specified; the default path is used automatically
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! ### 3. Construction using `new` (Shorthand)
//! Alternatively, you can instantiate the proxy directly using the short-hand
//! `new` constructor, which requires only the connection and destination:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::cache::CacheProxy;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//!
//! let bus_name = ":1.1001";
//!
//! let cache = CacheProxy::new(&conn, bus_name).await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [`get_items`]: CacheProxy#method.get_items
//! [`get_legacy_items`]: CacheProxy#method.get_legacy_items
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [add]: atspi_common::events::cache::AddAccessibleEvent
//! [remove]: atspi_common::events::cache::RemoveAccessibleEvent

use crate::common::{CacheItem, LegacyCacheItem};

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(
	interface = "org.a11y.atspi.Cache",
	default_path = "/org/a11y/atspi/cache",
	assume_defaults = false
)]
pub trait Cache {
	/// `GetItems` method
	fn get_items(&self) -> zbus::Result<Vec<CacheItem>>;

	/// `GetItems` method to support legacy servers (old Qt-based applications and old AT-SPI
	/// registry daemons)
	#[zbus(name = "GetItems")]
	fn get_legacy_items(&self) -> zbus::Result<Vec<LegacyCacheItem>>;
}
