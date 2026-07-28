//! # `CollectionProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Collection`
//! interface.
//!
//! The `Collection` interface is an can be a powerful query-engine for
//! accessibility builders. Instead of manually traversing the entire UI-tree
//! recursively (which requires making countless D-Bus calls), the
//! `Collection` interface allows clients to perform search queries
//! (like "find all focusable buttons") in a single D-Bus round-trip.
//!
//! Searches are configured using [`ObjectMatchRule`]s, which are easily
//! constructed using the [`ObjectMatchRuleBuilder`][mrb].
//!
//! ## D-Bus Addressing
//!
//! Since this interface is implemented dynamically on individual nodes within an
//! application's UI-tree, its D-Bus addressing (the unique bus name and object path)
//! varies per node. There is no static, well-known service destination or object path
//! applicable; address details must be resolved dynamically at runtime.
//!
//! ## How to obtain a `CollectionProxy`
//!
//! There are two idiomatic ways to obtain a `CollectionProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you already have an [`AccessibleProxy`][ap] pointing to a node that supports collections,
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
//! // Establish an `AccessibleProxy` for the node
//! let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! let root_node = obj_ref.into_accessible_proxy(&conn).await?;
//!
//! // Convert to `CollectionProxy` safely
//! let proxies = root_node.proxies().await?;
//! let collection = proxies.collection().await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! All proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting any P2P configuration if applicable.
//!
//! ### 2. Manual construction using the `builder`
//! If you know the exact D-Bus service destination and object path, you can
//! construct the proxy manually:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! use atspi_connection::AccessibilityConnection;
//! use atspi_proxies::collection::CollectionProxy;
//!
//! # block_on( async {
//! let a11y = AccessibilityConnection::new().await?;
//! let conn = a11y.connection();
//! use zbus::proxy::CacheProperties;
//!
//! let bus_name = ":1.1001";
//! let object_path = "/org/a11y/atspi/accessible/root";
//!
//! let collection = CollectionProxy::builder(&conn)
//!     .destination(bus_name)?
//!     .path(object_path)?
//!     .cache_properties(CacheProperties::No)
//!     .build()
//!     .await?;
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! ## Search Example
//!
//! This example demonstrates how to find all buttons inside a collection:
//!
//! ```rust,no_run
//! # use futures_lite::future::block_on;
//! # use atspi_connection::AccessibilityConnection;
//! # use atspi_proxies::proxy_ext::ProxyExt;
//! # use atspi_proxies::accessible::ObjectRefExt;
//! # use atspi_common::ObjectRefOwned;
//! use atspi_common::{ObjectMatchRule, SortOrder, MatchType, Role};
//!
//! # block_on( async {
//! # let a11y = AccessibilityConnection::new().await?;
//! # let conn = a11y.connection();
//! # let obj_ref = ObjectRefOwned::from_static_str_unchecked(":1.1000", "/org/a11y/atspi/accessible/root");
//! # let root_node = obj_ref.into_accessible_proxy(&conn).await?;
//! # let proxies = root_node.proxies().await?;
//! # let collection = proxies.collection().await?;
//! // 1. Build a search rule for Buttons
//! let rule = ObjectMatchRule::builder()
//!     .roles(&[Role::Button], MatchType::All)
//!     .build();
//!
//! // 2. Query the collection (0 means return all matches)
//! let matches = collection.get_matches(
//!     rule,
//!     SortOrder::Canonical,
//!     0,
//!     true, // traverse (not supported, pass true/false)
//! ).await?;
//!
//! for node_ref in matches {
//!     println!("Found matching actionable node: {node_ref:?}");
//! }
//! # Ok::<(), atspi_common::AtspiError>(())
//! # });
//! ```
//!
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [mrb]: atspi_common::object_match::ObjectMatchRuleBuilder

use crate::common::{ObjectMatchRule, ObjectRefOwned, SortOrder, TreeTraversalType};

// The proxy macro attribute `assume_defaults = false` to avoid generating defaults service and path
// The generated defaults don't make sense in AT-SPI2 / accessibility-bus context
// see:
// <https://docs.rs/crate/zbus_macros/5.11.0/source/src/proxy.rs#191-193>
#[zbus::proxy(interface = "org.a11y.atspi.Collection", assume_defaults = false)]
pub trait Collection {
	/// The active descendant of the given object.
	///
	/// May not be implemented by any known toolkit or private implementation.
	///
	/// See [atspi/collection.c](https://gitlab.gnome.org/GNOME/at-spi2-core/-/blob/main/atspi/atspi-collection.c?ref_type=heads#L272)
	///
	fn get_active_descendant(&self) -> zbus::Result<ObjectRefOwned>;

	/// Retrieves a list of objects that match the specified `ObjectMatchRule`, ordered according to `SortOrder` and limited by the count parameter.
	///
	/// # Arguments
	///
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported.
	///
	/// [`ObjectMatchRule`]: [`atspi_common::object_match::ObjectMatchRule`]
	/// [`SortOrder`]: [`atspi_common::SortOrder`]
	fn get_matches(
		&self,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// Retrieves objects from the collection, after `current_object`, matching a given `rule`.
	///
	/// # Arguments
	///
	/// * `current_object` - The object at which to start searching.
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `tree` - A [`TreeTraversalType`] specifying restrictions on the objects to be traversed.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported by the known implementation (atk-collection).
	///
	/// [`ObjectMatchRule`]: atspi_common::object_match::ObjectMatchRule
	/// [`SortOrder`]: atspi_common::SortOrder
	/// [`TreeTraversalType`]: atspi_common::TreeTraversalType
	fn get_matches_from(
		&self,
		current_object: &zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>>;

	/// Retrieves objects from the collection, before `current_object`, matching a given `rule`.
	///
	/// # Arguments
	///
	/// * `current_object` - The object at which to start searching.
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `tree` - A [`TreeTraversalType`] specifying restrictions on the objects to be traversed.
	/// * `limit_scope` - If `true`, only descendants of `current_object`'s parent will be returned.
	///    Otherwise (if `false`), any accessible may be returned if it would proceed `current_object` in a flattened hierarchy.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported by the known implementation (atk-collection).
	///
	/// [`ObjectMatchRule`]: atspi_common::object_match::ObjectMatchRule
	/// [`SortOrder`]: atspi_common::SortOrder
	/// [`TreeTraversalType`]: atspi_common::TreeTraversalType
	#[allow(clippy::too_many_arguments)]
	fn get_matches_to(
		&self,
		current_object: &zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		limit_scope: bool,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>>;
}
