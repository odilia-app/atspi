use atspi_common::{
	AtspiError, Interface, InterfaceSet, MatchType, ObjectMatchRule, ObjectRefOwned, Role,
	SortOrder, State, StateSet,
};
use core::time;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use crate::accessible::{AccessibleProxy, ObjectRefExt};

/// # [`TraversalHelper`]
///
/// A helper struct for clientside traversal of the accessibility tree.
///
/// Since most applications do not support the Collection interface,
/// [`TraversalHelper`] allows for clean traversals without needing to
/// implement tree algorithms yourself.
pub struct TraversalHelper<'a> {
	/// The root accessible from which to start the traversal
	pub root: AccessibleProxy<'a>,
	// The connection to use for creating zbus proxies
	pub conn: zbus::Connection,
	// The maximum depth to traverse in the accessibility tree; used to prevent excessively long traversals
	pub max_depth: u32,
	// The maximum time to traverse; used to prevent excessively long traversals
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

/// A helper for encapsulating all matching logic specified by the [`ObjectMatchRule`] for easier and reusable checks
struct ObjectMatchRuleHelper {
	attributes: Option<HashMap<String, String>>,
	states: Option<StateSet>,
	interfaces: Option<InterfaceSet>,
	role: Option<Role>,

	rule: ObjectMatchRule,
}

impl ObjectMatchRuleHelper {
	/// Given an [`AccessibleProxy`] and an [`ObjectMatchRule`], create a [`ObjectMatchRuleHelper`]
	/// which encapsulates all matching logic
	pub async fn from_accessible_and_rule(
		accessible: &AccessibleProxy<'_>,
		rule: ObjectMatchRule,
	) -> Result<ObjectMatchRuleHelper, Box<dyn std::error::Error>> {
		// If any of the match types is specified as [`MatchType::Invalid`], it means that
		// a search against the corresponding field should not be performed;
		// As such, the lookup for the corresponding field should be skipped to
		// prevent unnecessary dbus i/o
		let attributes = match rule.attr_mt {
			MatchType::Invalid => None,
			_ => Some(accessible.get_attributes().await?),
		};

		let states = match rule.states_mt {
			MatchType::Invalid => None,
			_ => Some(accessible.get_state().await?),
		};

		let interfaces = match rule.ifaces_mt {
			MatchType::Invalid => None,
			_ => Some(accessible.get_interfaces().await?),
		};

		let role = match rule.roles_mt {
			MatchType::Invalid => None,
			_ => Some(accessible.get_role().await?),
		};

		Ok(ObjectMatchRuleHelper { attributes, states, interfaces, role, rule })
	}

	/// The attributes of the root accessible matches the attribute match rule
	fn attributes_match(&self) -> Result<bool, AtspiError> {
		// if the match type is invalid, return true automatically without needing to check
		if self.rule.attr_mt == MatchType::Invalid {
			return Ok(true);
		}

		let attributes_in_accessible = match &self.attributes {
			Some(attributes) => attributes,
			// if the match type is invalid, return true automatically without needing to check
			// since an accessible can only have attributes and it can't be empty
			// these are all equivalent
			None => return Ok(true),
		};

		let mut matching_attributes = 0;

		for (expected_attribute, expected_value) in &self.rule.attr {
			if let Some(found_value) = attributes_in_accessible.get(expected_attribute) {
				if found_value == expected_value {
					matching_attributes += 1;
				}
			}
		}

		Ok(match self.rule.attr_mt {
			MatchType::Invalid => true,
			MatchType::All => matching_attributes == self.attributes.iter().count(),
			MatchType::Empty => matching_attributes == 0,
			MatchType::Any => matching_attributes > 0,
			MatchType::NA => matching_attributes == 0,
		})
	}

	/// The states of the root accessible match the state match rule
	fn states_match(&self) -> Result<bool, AtspiError> {
		// if the match type is invalid, return true automatically without needing to check
		if self.rule.states_mt == MatchType::Invalid {
			return Ok(true);
		}

		let mut matching_states = 0;

		let states_in_accessible = match self.states {
			Some(states) => states,
			None => return Err(AtspiError::Owned("States were not set".to_string())),
		};

		// a naive for loop is faster than using hashmap since the number of states is
		// known to be small
		for found_state in states_in_accessible {
			for expected_state in &self.rule.states {
				if found_state == expected_state {
					matching_states += 1;
				}
			}
		}

		Ok(match self.rule.ifaces_mt {
			MatchType::Invalid => true,
			MatchType::All => matching_states == self.states.iter().count(),
			MatchType::Empty => matching_states == 0,
			MatchType::Any => matching_states > 0,
			MatchType::NA => matching_states == 0,
		})
	}

	/// The interfaces of the root accessible match the interface match rule
	fn interfaces_match(&self) -> Result<bool, AtspiError> {
		// if the match type is invalid, return true automatically without needing to check
		if self.rule.ifaces_mt == MatchType::Invalid {
			return Ok(true);
		}

		let mut matching_interfaces = 0;

		let interfaces_in_accessible = match self.interfaces {
			Some(states) => states,
			None => return Err(AtspiError::Owned("Interfaces were not set".to_string())),
		};

		// a naive for loop is faster than using hashmap since the number of interfaces is
		// known to be small
		for found_interface in interfaces_in_accessible {
			for expected_interface in &self.rule.ifaces {
				if found_interface == expected_interface {
					matching_interfaces += 1;
				}
			}
		}

		Ok(match self.rule.ifaces_mt {
			MatchType::Invalid => true,
			MatchType::All => matching_interfaces == self.interfaces.iter().count(),
			MatchType::Empty => matching_interfaces == 0,
			MatchType::Any => matching_interfaces > 0,
			MatchType::NA => matching_interfaces == 0,
		})
	}

	/// The role of the root accessible matches the role match rule
	fn role_match(&self) -> Result<bool, AtspiError> {
		// if the match type is invalid, return true automatically without needing to check
		if self.rule.roles_mt == MatchType::Invalid {
			return Ok(true);
		}

		let role_in_accessible = match self.role {
			Some(role) => role,
			None => return Err(AtspiError::Owned("Role was not set".to_string())),
		};

		Ok(match self.rule.roles_mt {
			atspi_common::MatchType::Invalid => true,
			// since an accessible can only have one role and it can't be empty
			// these are all equivalent
			MatchType::All | MatchType::Empty | MatchType::Any => self.rule.roles.contains(&role_in_accessible),
			MatchType::NA => !self.rule.roles.contains(&role_in_accessible),
		})
	}

	/// All of the attributes, states, interfaces, and role in the accessible match the conditions of the [`ObjectMatchRule`]
	pub fn matches(&self) -> Result<bool, AtspiError> {
		let all_state_matches = self.attributes_match()?
			&& self.states_match()?
			&& self.interfaces_match()?
			&& self.role_match()?;
		Ok(match self.rule.invert {
			true => !all_state_matches,
			false => all_state_matches,
		})
	}
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
			if results.len() >= count as usize {
				break;
			}

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

			let matches_helper =
				match ObjectMatchRuleHelper::from_accessible_and_rule(&node, rule.clone()).await {
					Ok(matches_helper) => matches_helper,
					Err(e) => {
						return Err(AtspiError::Owned(e.to_string()));
					}
				};

			if matches_helper.matches()? {
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
