use atspi_common::{AtspiError, ObjectMatchRule, ObjectRefOwned, SortOrder, State};
use core::time;
use std::collections::VecDeque;
use std::time::Instant;

use crate::accessible::{AccessibleProxy, ObjectRefExt};

/// # [`TraversalHelper`]
///
/// A helper struct for clientside traversal of the accessibility tree.
///
/// Since most applications do not support the Collection interface,
/// `TraversalHelper` allows for clean traversals without needing to
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
	) -> impl std::future::Future<Output = Result<Vec<AccessibleProxy<'_>>, AtspiError>> + Send;
}

impl CollectionClientside for TraversalHelper<'_> {
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

	async fn get_matches(
		&self,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		count: i32,
		traverse: bool,
	) -> Result<Vec<AccessibleProxy<'_>>, AtspiError> {
		if traverse {
			return Err(AtspiError::Owned("Traverse not supported".to_string()));
		}

		if count < 0 {
			return Err(AtspiError::Owned("Count must be non-negative".to_string()));
		}

		let start = Instant::now();
		let mut results: Vec<AccessibleProxy<'_>> = Vec::new();
		let mut queue: VecDeque<(AccessibleProxy<'_>, u32)> = VecDeque::new();

		queue.push_back((self.root.clone(), 0u32));

		while let Some((node, depth)) = queue.pop_front() {
			// Time limit
			if let Some(max_time) = self.max_time {
				if start.elapsed() > max_time {
					return Err(AtspiError::TraversalTimeoutError);
				}
			}

			// Depth limit
			if depth > self.max_depth {
				continue;
			}

			let attr_matches = match rule.attr_mt {
				atspi_common::MatchType::Invalid => true,
				atspi_common::MatchType::All | atspi_common::MatchType::Empty => {
					panic!("Not implemented")
				}
				atspi_common::MatchType::Any => panic!("Not implemented"),
				atspi_common::MatchType::NA => panic!("Not implemented"),
			};

			let state_matches = match rule.states_mt {
				atspi_common::MatchType::Invalid => true,
				atspi_common::MatchType::All | atspi_common::MatchType::Empty => {
					panic!("Not implemented")
				}
				atspi_common::MatchType::Any => panic!("Not implemented"),
				atspi_common::MatchType::NA => panic!("Not implemented"),
			};

			let role_matches = match rule.roles_mt {
				atspi_common::MatchType::Invalid => true,
				atspi_common::MatchType::All | atspi_common::MatchType::Empty => {
					panic!("Not implemented")
				}
				atspi_common::MatchType::Any => panic!("Not implemented"),
				atspi_common::MatchType::NA => panic!("Not implemented"),
			};

			let iface_matches = match rule.ifaces_mt {
				atspi_common::MatchType::Invalid => true,
				atspi_common::MatchType::All | atspi_common::MatchType::Empty => {
					panic!("Not implemented")
				}
				atspi_common::MatchType::Any => panic!("Not implemented"),
				atspi_common::MatchType::NA => panic!("Not implemented"),
			};

			let add_node = if rule.invert {
				!(attr_matches && state_matches && role_matches && iface_matches)
			} else {
				attr_matches && state_matches && role_matches && iface_matches
			};

			if add_node {
				results.push(node.clone());
			}

			// Traverse children
			let children = node.get_children().await?;
			for child in children {
				if !child.is_null() {
					queue.push_back((child.into_accessible_proxy(&self.conn).await?, depth + 1));
				}
			}
		}

		// Sort results
		match sortby {
			SortOrder::Canonical => Ok(results),
			_ => {
				return Err(AtspiError::Owned(format!("Unsupported SortOrder: {:?}", sortby)));
			}
		}
	}
}
