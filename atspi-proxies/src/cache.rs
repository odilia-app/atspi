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
//! ## Defaults
//!
//! The `Cache` interface is implemented for a single, central object in the
//! application's UI-tree. Because this object path is fixed across all applications,
//! the proxy defines a fixed default path: `/org/a11y/atspi/cache`.
//!
//! ## How to obtain a `CacheProxy`
//!
//! There are three idiomatic ways to obtain a `CacheProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you have an [`AccessibleProxy`][ap] pointing to the application's root node,
//! you can query its interfaces and convert it safely:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = root_node.proxies().await?;
//! let cache = proxies.cache().await?;
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
//! ```rust,ignore
//! let cache = CacheProxy::builder(&connection)
//!     .destination(bus_name)?
//!     // No path is specified; the default path is used automatically
//!     .build()
//!     .await?;
//! ```
//!
//! ### 3. Construction using `new` (Shorthand)
//! Alternatively, you can instantiate the proxy directly using the short-hand
//! `new` constructor, which requires only the connection and destination:
//!
//! ```rust,ignore
//! let cache = CacheProxy::new(&connection, service_name).await?;
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
