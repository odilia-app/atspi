use atspi_common::{AtspiError, ObjectMatchRule, SortOrder, State};
use core::time;
use std::collections::VecDeque;
use std::time::Instant;

use crate::accessible::{AccessibleProxy, ObjectRefExt};

/// # [`TraversalHelper`]
///
/// A helper struct for clientside traversal of the accessibility tree.
///
/// Since most applications do not support the `org.a11y.atspi.Collection` interface,
/// `TraversalHelper` allows for a simpler traversal abstraction without needing to
/// implement tree algorithms yourself.
pub struct TraversalHelper<'a> {
	/// The root from which to start the traversal
	pub root: AccessibleProxy<'a>,
	// The connection to use for creating proxies
	pub conn: zbus::Connection,
	// The maximum depth to traverse to prevent excessively long traversals
	pub max_depth: u32,
	// The maximum time to traverse to prevent excessively long traversals
	pub max_time: Option<time::Duration>,
}

impl<'a> TraversalHelper<'a> {
	#[must_use]
	pub fn new(
		root: AccessibleProxy<'a>,
		conn: zbus::Connection,
		max_depth: u32,
		max_time: Option<time::Duration>,
	) -> TraversalHelper<'a> {
		TraversalHelper { root, conn, max_depth, max_time }
	}
}

/// A trait which mimics the `org.a11y.atspi.Collection` interface
/// but entirely clientside. All methods perform dbus roundtrips.
pub trait CollectionClientside {
	/// Find the closest Accessible with [`atspi_common::State::Active`] that is a descendant of the root object
	fn get_active_descendant(
		&self,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;

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
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;
}

impl CollectionClientside for TraversalHelper<'_> {
	/// Find the closest Accessible with [`atspi_common::State::Active`] that is a descendant of the root object
	async fn get_active_descendant(&self) -> Result<AccessibleProxy<'_>, AtspiError> {
		let root = &self.root;
		let mut queue = VecDeque::new();

		let start = Instant::now();

		queue.push_back((root.clone(), 0));

		while let Some((node, current_depth)) = queue.pop_front() {
			if let Some(max_time) = self.max_time {
				if start.elapsed() > max_time {
					return Err(AtspiError::TraversalTimeoutError);
				}
			}
			if current_depth > self.max_depth {
				continue;
			}

			if node.get_state().await?.contains(State::Active) {
				return Ok(node);
			}

			let children = node.get_children().await?;
			for child in children {
				if !child.is_null() {
					queue.push_back((
						child.clone().into_accessible_proxy(&self.conn).await?,
						current_depth + 1,
					));
				}
			}
		}

		Err(AtspiError::Owned("Could not find active descendant".to_string()))
	}
}
