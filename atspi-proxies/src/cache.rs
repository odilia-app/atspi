//! # `CacheProxy`
//!
//! The `org.a11y.atspi.Cache` interface provides a way to cache
//! `CacheItem`s for use by AT-SPI clients, such as screen-readers.
//!
//! `CacheItem`s are sent as part of [`AddAccessibleEvent`][cae]s which may
//! be used to update a screen-reader's cache.
//!
//! This interface also implements a method to help populate
//! the cache with `CacheItem`s, representing the UI-tree.
//!
//! ## Defaults
//!
//! "org.a11y.atspi.Cache" may be implemented for a single `cache` path
//! in the application's UI-tree.
//!
//! Service should be provided by the builder or inherited from the
//! [`zbus::Proxy`] this `CacheProxy` is derived from.
//!
//! No default service makes sense for this proxy, thus the macro is
//! instructed explicitly not to generate the defaults.
//!
//! [cae]: atspi_common::events::cache::AddAccessibleEvent

use crate::common::{CacheItem, LegacyCacheItem};

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
