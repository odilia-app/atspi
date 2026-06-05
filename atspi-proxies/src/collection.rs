//! # `CollectionProxy`
//!
//! A handle for a remote object implementing the `org.a11y.atspi.Collection`
//! interface.
//!
//! The `Collection` interface is an incredibly powerful query-engine for
//! accessibility builders. Instead of manually traversing the entire UI-tree
//! recursively (which requires making countless synchronous D-Bus calls), the
//! `Collection` interface allows clients to perform advanced search queries
//! (like "find all focusable buttons") in a single D-Bus round-trip.
//!
//! Searches are configured using [`ObjectMatchRule`]s, which are easily
//! constructed using the [`ObjectMatchRuleBuilder`][mrb].
//!
//! ## Defaults
//!
//! The `Collection` interface may be implented for any arbitrary node in the UI-tree.
//! This means the bus name and path will vary.
//!
//! ## How to obtain a `CollectionProxy`
//!
//! There are three idiomatic ways to obtain a `CollectionProxy`:
//!
//! ### 1. Safe conversion via [`ProxyExt`][pe] (Recommended)
//! If you have an [`AccessibleProxy`][ap] pointing to the application's root node,
//! you can query its interfaces and convert it safely:
//!
//! ```rust,ignore
//! use atspi::ProxyExt;
//!
//! let proxies = root_node.proxies().await?;
//! let collection = proxies.collection().await?;
//! ```
//!
//! All proxies obtained through [`ProxyExt`][pe] share their underlying
//! [`zbus::Connection`], inheriting any P2P configuration if applicable.
//!
//! ### 2. Manual construction using the `builder` (Fixed Path)
//! Because the object path of the `Collection` interface is fixed, you only need
//! to supply the application's unique D-Bus service destination. The builder will
//! automatically use the default path:
//!
//! ```rust,ignore
//! let collection = CollectionProxy::builder(&connection)
//!     .destination(service_name)?
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
//! let collection = CollectionProxy::new(&connection, service_name).await?;
//! ```
//!
//! ## Search Example
//!
//! This example demonstrates how to find all buttons inside a collection:
//!
//! ```rust,ignore
//! use atspi::{ObjectMatchRule, SortOrder, MatchType, Role};
//!
//! // 1. Build a search rule for PushButtons
//! let rule = ObjectMatchRule::builder()
//!     .roles(&[Role::PushButton], MatchType::All)
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
//! ```
//!
//! [pe]: crate::proxy_ext::ProxyExt
//! [ap]: crate::accessible::AccessibleProxy
//! [mrb]: atspi-common::object_match::ObjectMatchRuleBuilder

use atspi_common::object_ref::ObjectRefOwned;

use crate::common::{ObjectMatchRule, SortOrder, TreeTraversalType};
// We don't want the proxy macro to auto-derive
// defaults, so assume_defaults is explicitly set to false.
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
	///    Otherwise (if `false`), any accessible may be returned if it would preceed `current_object` in a flattened hierarchy.
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
