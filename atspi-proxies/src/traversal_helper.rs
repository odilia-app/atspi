use atspi_common::{AtspiError, State};
use core::time;
use std::collections::VecDeque;
use std::time::Instant;

use crate::accessible::{AccessibleProxy, ObjectRefExt};

/// # [`TraversalHelper`]
///
/// A helper struct for clientside traversal of the accessibility tree.
///
/// Since most applications do not support the Collection interface,
/// TraversalHelper allows for clean traversals without needing to
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

/// A trait which mimics the org.a11y.atspi.Collection interface
/// but entirely clientside. All methods perform dbus roundtrips.
pub trait CollectionClientside {
	/// Find the closest Accessible with State:Active starting from the root object
	fn get_active_descendant(
		&self,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;
}

impl<'a> CollectionClientside for TraversalHelper<'a> {
	/// Find the closest Accessible with State:Active starting from the root object
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
