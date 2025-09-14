//! # [`CollectionProxy`]
//!
//! A handle to a remote object implementing the `org.a11y.atspi.Collection`
//! interface.
//!
//! `Collection` is the interface which is implemented by objects that contain
//! other objects, such as a window or a table.
//!
//! See the documentation on the individual methods for details:
//!
//! * [`get_matches`](struct.CollectionProxy.html#method.get_matches)
//! * [`get_matches_from`](struct.CollectionProxy.html#method.get_matches_from)
//! * [`get_matches_to`](struct.CollectionProxy.html#method.get_matches_to)
//!
//! [`CollectionProxy`]: crate::collection::CollectionProxy

use crate::accessible::AccessibleProxy;
use crate::common::{ObjectMatchRule, SortOrder, TreeTraversalType};
use atspi_common::object_ref::ObjectRefOwned;

// #[zbus::proxy(interface = "org.a11y.atspi.Collection", assume_defaults = true)]

pub(crate) struct NativeCollection;

#[zbus::interface(
	name = "org.a11y.atspi.Collection",
	introspection_docs = false,
	proxy(visibility = "pub(crate)")
)]
impl NativeCollection {
	/// The active descendant of the given object.
	///
	/// May not be implemented by any known toolkit or private implementation.
	///
	/// See [atspi/collection.c](https://gitlab.gnome.org/GNOME/at-spi2-core/-/blob/main/atspi/atspi-collection.c?ref_type=heads#L272)
	///
	fn get_active_descendant(&self) -> zbus::fdo::Result<ObjectRefOwned> {
		Ok(ObjectRefOwned::default())
	}

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
	#[allow(unused_variables)]
	fn get_matches(
		&self,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		count: i32,
		traverse: bool,
	) -> zbus::fdo::Result<Vec<ObjectRefOwned>> {
		Ok(Vec::new())
	}

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
	#[allow(unused_variables)]
	fn get_matches_from(
		&self,
		current_object: zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		count: i32,
		traverse: bool,
	) -> zbus::fdo::Result<Vec<ObjectRefOwned>> {
		Ok(Vec::new())
	}

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
	//	#[allow(clippy::too_many_arguments)]
	#[allow(unused_variables)]
	fn get_matches_to(
		&self,
		current_object: zbus::zvariant::ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		limit_scope: bool,
		count: i32,
		traverse: bool,
	) -> zbus::fdo::Result<Vec<ObjectRefOwned>> {
		Ok(Vec::new())
	}
}

pub struct CollectionProxy<'a> {
	accessible: AccessibleProxy<'a>,
	collection: Option<NativeCollectionProxy<'a>>,
}

impl<'a> CollectionProxy<'a> {
	/// Create a new instance of `CollectionProxy` from an `AccessibleProxy`.
	pub async fn new(accessible: AccessibleProxy<'a>) -> zbus::Result<Self> {
		let proxy: &zbus::Proxy = accessible.inner();
		let conn: &zbus::Connection = proxy.connection();

		let mut collection: Option<NativeCollectionProxy<'a>> = None;
		let iface_set = accessible.get_interfaces().await?;

		if iface_set.contains(crate::Interface::Collection) {
			collection = match NativeCollectionProxy::builder(conn)
				.path(proxy.path())?
				.destination(proxy.destination())?
				.build()
				.await
			{
				Ok(proxy) => Some(proxy),
				Err(e) if should_fallback(&e) => None,
				Err(e) => return Err(e),
			};
		}

		Ok(Self { accessible, collection })
	}

	pub async fn get_matches(
		&self,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>> {
		match self
			.collection
			.get_matches(rule.clone(), sortby, count, traverse)
			.await
		{
			Ok(v) => Ok(v),
			Err(e) if should_fallback(&e) => {
				self.fallback_get_matches(rule, sortby, count, traverse).await
			}
			Err(e) => Err(e),
		}
	}

	async fn fallback_get_matches(
		&self,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>> {
		let children = self.accessible.get_children().await?;
		let mut results = Vec::new();

		for child in children {
			// MatchRule::matches needs to be implemented
			if rule.matches(&child).await? {
				results.push(child);
			}
		}

		// Sorting logic based on SortOrder
		match sortby {
			SortOrder::None => {}
			SortOrder::Ascending => {
				results.sort_by(|a, b| a.cmp(b));
			}
			SortOrder::Descending => {
				results.sort_by(|a, b| b.cmp(a));
			}
		}

		if count > 0 && results.len() as i32 > count {
			results.truncate(count as usize);
		}

		Ok(results)
	}

	pub async fn get_matches_from(
		&self,
		current_object: ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>> {
		match self
			.native
			.get_matches_from(&current_object, rule.clone(), sortby, tree, count, traverse)
			.await
		{
			Ok(v) => Ok(v),
			Err(e) if should_fallback(&e) => {
				self.fallback_get_matches_from(current_object, rule, sortby, tree, count, traverse)
					.await
			}
			Err(e) => Err(e),
		}
	}

	pub async fn get_matches_to(
		&self,
		current_object: ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		limit_scope: bool,
		count: i32,
		traverse: bool,
	) -> zbus::Result<Vec<ObjectRefOwned>> {
		match self
			.native
			.get_matches_to(
				&current_object,
				rule.clone(),
				sortby,
				tree,
				limit_scope,
				count,
				traverse,
			)
			.await
		{
			Ok(v) => Ok(v),
			Err(e) if should_fallback(&e) => {
				self.fallback_get_matches_to(
					current_object,
					rule,
					sortby,
					tree,
					limit_scope,
					count,
					traverse,
				)
				.await
			}
			Err(e) => Err(e),
		}
	}
}

fn should_fallback(e: &zbus::Error) -> bool {
	match e {
		zbus::Error::MethodError(name, ..) => {
			let n = name.as_str();
			n == "org.freedesktop.DBus.Error.UnknownMethod"
				|| n == "org.freedesktop.DBus.Error.NotSupported"
				|| n == "org.freedesktop.DBus.Error.Failed"
		}
		_ => false,
	}
}
