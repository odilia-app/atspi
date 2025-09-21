use atspi_common::{AtspiError, State};
use core::time;
use std::collections::VecDeque;

use crate::accessible::{AccessibleProxy, ObjectRefExt};

pub struct TraversalHelper {
	pub root: AccessibleProxy<'static>,
	pub conn: zbus::Connection,
	pub max_depth: u32,
	pub max_time: time::Duration,
}

impl TraversalHelper {
	#[must_use]
	pub fn new(
		root: AccessibleProxy<'static>,
		conn: zbus::Connection,
		max_depth: u32,
		max_time: time::Duration,
	) -> TraversalHelper {
		TraversalHelper { root, conn, max_depth, max_time }
	}
}

// A set of helper methods that allow for simpler clientside traversal
// of the accessibility tree. It is used sed since many applications
// do not support the Collection proxy and thus we need to performan
// the traversal ourselves
pub trait TraversalExt {
	/// The active descendant of the given object.
	fn get_active_descendant(
		&self,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;
}

impl TraversalExt for TraversalHelper {
	async fn get_active_descendant(&self) -> Result<AccessibleProxy<'_>, AtspiError> {
		let root = &self.root;
		let mut queue = VecDeque::new();

		queue.push_back((root.clone(), 0));

		while let Some((node, current_depth)) = queue.pop_front() {
			if current_depth > self.max_depth {
				continue;
			}

			let state = node.get_state().await?;
			if state.contains(State::Active) {
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
