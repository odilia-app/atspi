use atspi_common::{
	AtspiError, InterfaceSet, MatchType, ObjectMatchRule, Role, SortOrder, State, StateSet,
	TreeTraversalType,
};
use core::time;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use zbus::zvariant::ObjectPath;

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
///
/// The *canonical* order these methods speak of is the order a depth-first, pre-order walk
/// visits objects in. Only [`SortOrder::Canonical`] and [`SortOrder::ReverseCanonical`] are
/// supported, as they are the only orders the known serverside implementation supports.
///
/// Every traversal is bounded by the helper's `max_time`, and every descent by its
/// `max_depth`.
pub trait CollectionClientside {
	/// Find the closest Accessible with State:Active starting from the root object
	///
	/// # Errors
	///
	/// Returns an error if a dbus call fails, if the traversal exceeds `max_time`, or if no
	/// descendant of the root is active.
	fn get_active_descendant(
		&self,
	) -> impl std::future::Future<Output = Result<AccessibleProxy<'_>, AtspiError>> + Send;

	/// Retrieves a list of objects that match the specified `ObjectMatchRule`, ordered according to `SortOrder` and limited by the count parameter.
	///
	/// The root object itself is a candidate for matching, as are all of its descendants.
	///
	/// # Arguments
	///
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported.
	///
	/// # Errors
	///
	/// Returns an error if `traverse` is set, if `count` is negative, if `sortby` is
	/// unsupported, if a dbus call fails, or if the traversal exceeds `max_time`.
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

	/// Retrieves objects after `current_object`, matching a given `rule`.
	///
	/// As on the `Collection` interface, `current_object` is addressed by object path alone;
	/// it is resolved against the bus name of the traversal root, of which it must be the
	/// root itself or a descendant.
	///
	/// # Arguments
	///
	/// * `current_object` - The object at which to start searching; it is never itself returned.
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `tree` - A [`TreeTraversalType`] restricting which objects are visited: everything
	///   that follows `current_object` in canonical order, only its children, or only the
	///   siblings that follow it.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported.
	///
	/// # Errors
	///
	/// Returns an error if `traverse` is set, if `count` is negative, if `sortby` is
	/// unsupported, if `current_object` is not the root or one of its descendants, if a dbus
	/// call fails, or if the traversal exceeds `max_time`.
	fn get_matches_from(
		&self,
		current_object: &ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		count: i32,
		traverse: bool,
	) -> impl std::future::Future<Output = Result<Vec<AccessibleProxy<'_>>, AtspiError>> + Send;

	/// Retrieves objects *before* `current_object`, matching a given `rule`.
	///
	/// The tree is walked backwards from `current_object`, so a non-zero `count` yields the
	/// matches *nearest* to it. `current_object` is addressed as in
	/// [`get_matches_from`](Self::get_matches_from).
	///
	/// # Arguments
	///
	/// * `current_object` - The object at which to start searching; it is never itself returned.
	/// * `rule` - An [`ObjectMatchRule`] describing the match criteria.
	/// * `sortby` - A [`SortOrder`] specifying the way the results are to be sorted.
	/// * `tree` - A [`TreeTraversalType`] restricting which objects are visited: everything
	///   that precedes `current_object` in canonical order - its ancestors included - only
	///   its children, or only the siblings that precede it. Note that the restricted types
	///   select which objects are visited, not where they sit relative to `current_object`.
	/// * `limit_scope` - If `true`, only descendants of `current_object`'s parent will be returned.
	///   Otherwise (if `false`), any accessible may be returned if it would proceed `current_object` in a flattened hierarchy.
	/// * `count` - The maximum number of results to return, or 0 for no limit.
	/// * `traverse` - Not supported.
	///
	/// # Errors
	///
	/// Returns an error if `traverse` is set, if `count` is negative, if `sortby` is
	/// unsupported, if `current_object` is not the root or one of its descendants, if a dbus
	/// call fails, or if the traversal exceeds `max_time`.
	#[allow(clippy::too_many_arguments)]
	fn get_matches_to(
		&self,
		current_object: &ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		limit_scope: bool,
		count: i32,
		traverse: bool,
	) -> impl std::future::Future<Output = Result<Vec<AccessibleProxy<'_>>, AtspiError>> + Send;
}

/// The wall-clock budget of a single traversal.
struct Deadline {
	start: Instant,
	max_time: Option<time::Duration>,
}

impl Deadline {
	fn new(max_time: Option<time::Duration>) -> Deadline {
		Deadline { start: Instant::now(), max_time }
	}

	/// Returns [`AtspiError::TraversalTimeoutError`] once the budget is spent.
	fn check(&self) -> Result<(), AtspiError> {
		match self.max_time {
			Some(max_time) if self.start.elapsed() > max_time => {
				Err(AtspiError::TraversalTimeoutError)
			}
			_ => Ok(()),
		}
	}
}

/// The parameters that remain constant for the duration of one search.
struct Search {
	matcher: ObjectMatchRuleHelper,
	/// The maximum number of matches to collect; [`None`] means "no limit".
	limit: Option<usize>,
	deadline: Deadline,
}

impl Search {
	fn new(
		rule: ObjectMatchRule,
		count: i32,
		max_time: Option<time::Duration>,
	) -> Result<Search, AtspiError> {
		// The `Collection` interface uses a count of 0 to mean "no limit"
		let limit = match count {
			0 => None,
			count if count.is_negative() => {
				return Err(AtspiError::Owned("Count must be non-negative".to_string()))
			}
			count => Some(usize::try_from(count).map_err(AtspiError::IntConversionError)?),
		};

		Ok(Search {
			matcher: ObjectMatchRuleHelper::new(rule),
			limit,
			deadline: Deadline::new(max_time),
		})
	}

	/// Whether enough matches have been collected to stop the traversal.
	fn is_satisfied_by(&self, results: &[AccessibleProxy<'_>]) -> bool {
		self.limit.is_some_and(|limit| results.len() >= limit)
	}
}

/// A helper for encapsulating all matching logic specified by the [`ObjectMatchRule`] for easier and reusable checks
struct ObjectMatchRuleHelper {
	rule: ObjectMatchRule,
}

impl ObjectMatchRuleHelper {
	fn new(rule: ObjectMatchRule) -> ObjectMatchRuleHelper {
		ObjectMatchRuleHelper { rule }
	}

	/// Evaluate one criterion of the rule.
	///
	/// * `matched` - how many of the items the rule asks for the object actually has.
	/// * `expected` - how many items the rule asks for.
	/// * `object_set_is_empty` - whether the object's own set of items is empty.
	fn evaluate(
		match_type: MatchType,
		matched: usize,
		expected: usize,
		object_set_is_empty: bool,
	) -> bool {
		match match_type {
			// An invalid match type means the criterion is not applied at all
			MatchType::Invalid => true,
			MatchType::All => matched == expected,
			MatchType::Any => matched > 0,
			MatchType::None => matched == 0,
			// `Empty` is the same as `All` for a non-empty criterion; for an empty one it
			// additionally requires the object's own set to be empty
			MatchType::Empty => {
				if expected == 0 {
					object_set_is_empty
				} else {
					matched == expected
				}
			}
		}
	}

	/// The attributes of the accessible match the attribute match rule
	fn attributes_match(&self, attributes: &HashMap<String, String>) -> bool {
		let matching_attributes = self
			.rule
			.attr
			.iter()
			.filter(|(expected_attribute, expected_value)| {
				attributes.get(*expected_attribute) == Some(*expected_value)
			})
			.count();

		Self::evaluate(
			self.rule.attr_mt,
			matching_attributes,
			self.rule.attr.len(),
			attributes.is_empty(),
		)
	}

	/// The states of the accessible match the state match rule
	fn states_match(&self, states: StateSet) -> bool {
		// a naive iteration is faster than using a hashmap since the number of states is
		// known to be small
		let expected = self.rule.states.iter().count();
		let matching_states = self
			.rule
			.states
			.iter()
			.filter(|state| states.contains(*state))
			.count();

		Self::evaluate(self.rule.states_mt, matching_states, expected, states.is_empty())
	}

	/// The interfaces of the accessible match the interface match rule
	fn interfaces_match(&self, interfaces: InterfaceSet) -> bool {
		// a naive iteration is faster than using a hashmap since the number of interfaces is
		// known to be small
		let expected = self.rule.ifaces.iter().count();
		let matching_interfaces = self
			.rule
			.ifaces
			.iter()
			.filter(|iface| interfaces.contains(*iface))
			.count();

		Self::evaluate(self.rule.ifaces_mt, matching_interfaces, expected, interfaces.bits() == 0)
	}

	/// The role of the accessible matches the role match rule
	fn role_matches(&self, role: Role) -> bool {
		// An accessible has exactly one role, so asking for "all" of several roles can only
		// sensibly mean "one of them", and its set of roles is never empty
		let contains = self.rule.roles.contains(role);

		match self.rule.roles_mt {
			MatchType::Invalid => true,
			// Asking for all of no roles at all matches every accessible
			MatchType::All => self.rule.roles.is_empty() || contains,
			MatchType::Any => contains,
			MatchType::None => !contains,
			MatchType::Empty => !self.rule.roles.is_empty() && contains,
		}
	}

	/// All of the attributes, states, interfaces, and role in the accessible match the conditions of the [`ObjectMatchRule`]
	///
	/// Only those properties the rule actually inspects are requested over dbus; a criterion
	/// whose match type is [`MatchType::Invalid`] is skipped entirely, and evaluation stops
	/// at the first criterion that fails.
	async fn matches(&self, accessible: &AccessibleProxy<'_>) -> Result<bool, AtspiError> {
		let mut matches = true;

		if matches && self.rule.roles_mt != MatchType::Invalid {
			matches = self.role_matches(accessible.get_role().await?);
		}
		if matches && self.rule.states_mt != MatchType::Invalid {
			matches = self.states_match(accessible.get_state().await?);
		}
		if matches && self.rule.ifaces_mt != MatchType::Invalid {
			matches = self.interfaces_match(accessible.get_interfaces().await?);
		}
		if matches && self.rule.attr_mt != MatchType::Invalid {
			matches = self.attributes_match(&accessible.get_attributes().await?);
		}

		Ok(if self.rule.invert { !matches } else { matches })
	}
}

/// Private traversal primitives shared by the [`CollectionClientside`] methods.
impl TraversalHelper<'_> {
	/// Whether both proxies address the same object on the bus.
	fn is_same_object(left: &AccessibleProxy<'_>, right: &AccessibleProxy<'_>) -> bool {
		left.inner().destination() == right.inner().destination()
			&& left.inner().path() == right.inner().path()
	}

	/// Resolve an object path into an [`AccessibleProxy`] on the same bus name as the root.
	///
	/// The `Collection` interface addresses `current_object` by object path alone, since a
	/// collection only ever searches within a single application; the same holds here.
	async fn accessible_at_path<'p>(
		&'p self,
		path: &ObjectPath<'_>,
	) -> Result<AccessibleProxy<'p>, AtspiError> {
		AccessibleProxy::builder(&self.conn)
			.destination(self.root.inner().destination().to_owned())?
			.path(path.to_owned())?
			.cache_properties(zbus::proxy::CacheProperties::No)
			.build()
			.await
			.map_err(AtspiError::from)
	}

	/// The non-null children of `node`, in canonical order.
	async fn children_of<'p>(
		&'p self,
		node: &AccessibleProxy<'_>,
	) -> Result<Vec<AccessibleProxy<'p>>, AtspiError> {
		let mut children = Vec::new();
		for child in node.get_children().await? {
			if child.is_null() {
				continue;
			}
			children.push(child.into_accessible_proxy(&self.conn).await?);
		}
		Ok(children)
	}

	/// The parent of `node`, or [`None`] if it does not have one.
	async fn parent_of<'p>(
		&'p self,
		node: &AccessibleProxy<'_>,
	) -> Result<Option<AccessibleProxy<'p>>, AtspiError> {
		let parent = node.parent().await?;
		if parent.is_null() || parent.name().is_none() {
			return Ok(None);
		}
		Ok(Some(parent.into_accessible_proxy(&self.conn).await?))
	}

	/// The ancestors of `node`, from its parent up to and including the traversal root,
	/// together with the depth of `node` relative to that root.
	///
	/// Returns an error if `node` is not the root or one of its descendants within `max_depth`.
	async fn ancestors_to_root<'p>(
		&'p self,
		node: &AccessibleProxy<'p>,
		deadline: &Deadline,
	) -> Result<(Vec<AccessibleProxy<'p>>, u32), AtspiError> {
		let mut ancestors = Vec::new();
		let mut depth = 0u32;
		let mut current = node.clone();

		while !Self::is_same_object(&current, &self.root) {
			deadline.check()?;

			if depth >= self.max_depth {
				return Err(AtspiError::Owned(
					"`current_object` is deeper below the traversal root than `max_depth`"
						.to_string(),
				));
			}

			let Some(parent) = self.parent_of(&current).await? else {
				return Err(AtspiError::Owned(
					"`current_object` is not the traversal root or one of its descendants"
						.to_string(),
				));
			};

			ancestors.push(parent.clone());
			current = parent;
			depth += 1;
		}

		Ok((ancestors, depth))
	}

	/// The siblings of `child` that precede it and those that follow it, in canonical order.
	///
	/// An object without a parent - the traversal root, for instance - has no siblings.
	async fn siblings_around<'p>(
		&'p self,
		child: &AccessibleProxy<'_>,
		parent: Option<&AccessibleProxy<'_>>,
	) -> Result<(Vec<AccessibleProxy<'p>>, Vec<AccessibleProxy<'p>>), AtspiError> {
		let Some(parent) = parent else {
			return Ok((Vec::new(), Vec::new()));
		};

		let siblings = self.children_of(parent).await?;
		let index = siblings
			.iter()
			.position(|sibling| Self::is_same_object(sibling, child))
			.ok_or_else(|| {
				AtspiError::Owned(
					"the accessible tree changed during traversal: an object is not among the children of its own parent"
						.to_string(),
				)
			})?;

		let (preceding, rest) = siblings.split_at(index);
		Ok((preceding.to_vec(), rest[1..].to_vec()))
	}

	/// The continuation of a canonical order walk directly after `current`.
	///
	/// The result is a depth-first stack: the node that follows `current` is at the end.
	/// `ancestors` are the ancestors of `current` as returned by [`Self::ancestors_to_root`].
	async fn continuation_after<'p>(
		&'p self,
		current: &AccessibleProxy<'p>,
		current_depth: u32,
		ancestors: &[AccessibleProxy<'p>],
		deadline: &Deadline,
	) -> Result<Vec<(AccessibleProxy<'p>, u32)>, AtspiError> {
		// Once the subtree of `current` is exhausted, the walk resumes with the siblings
		// that follow it, then with the siblings that follow its parent, and so on up to
		// the root. Collect those groups from the deepest level upwards.
		let mut following_per_level = Vec::with_capacity(ancestors.len());
		let mut child = current.clone();
		let mut depth = current_depth;

		for ancestor in ancestors {
			deadline.check()?;
			let (_, following) = self.siblings_around(&child, Some(ancestor)).await?;
			following_per_level.push((depth, following));
			child = ancestor.clone();
			depth -= 1;
		}

		// The shallowest group is visited last, so it goes to the bottom of the stack;
		// within a group, the first sibling is visited first, so it goes on top.
		let mut stack = Vec::new();
		for (depth, following) in following_per_level.into_iter().rev() {
			for sibling in following.into_iter().rev() {
				stack.push((sibling, depth));
			}
		}

		// The children of `current` are visited before anything else.
		if current_depth < self.max_depth {
			for child in self.children_of(current).await?.into_iter().rev() {
				stack.push((child, current_depth + 1));
			}
		}

		Ok(stack)
	}

	/// The node that precedes `node` in canonical order, together with its depth.
	///
	/// Returns [`None`] once the walk reaches `scope_root`, the shallowest object it may visit.
	async fn preorder_predecessor<'p>(
		&'p self,
		node: &AccessibleProxy<'_>,
		depth: u32,
		scope_root: &AccessibleProxy<'_>,
	) -> Result<Option<(AccessibleProxy<'p>, u32)>, AtspiError> {
		if Self::is_same_object(node, scope_root) {
			return Ok(None);
		}

		let Some(parent) = self.parent_of(node).await? else {
			return Ok(None);
		};

		let (preceding, _) = self.siblings_around(node, Some(&parent)).await?;

		// A node is directly preceded by its parent when it is the first child
		let Some(previous_sibling) = preceding.last() else {
			return Ok(Some((parent, depth.saturating_sub(1))));
		};

		// Otherwise it is preceded by the last, deepest descendant of the previous sibling
		let mut candidate = previous_sibling.clone();
		let mut candidate_depth = depth;
		while candidate_depth < self.max_depth {
			let children = self.children_of(&candidate).await?;
			let Some(last_child) = children.last() else {
				break;
			};
			candidate = last_child.clone();
			candidate_depth += 1;
		}

		Ok(Some((candidate, candidate_depth)))
	}

	/// Walk forwards in canonical order from a seeded depth-first stack, collecting matches.
	///
	/// Each entry of the stack carries the depth of its node relative to the traversal root,
	/// which is what bounds the descent to `max_depth`.
	async fn collect_from_stack<'p>(
		&'p self,
		search: &Search,
		mut stack: Vec<(AccessibleProxy<'p>, u32)>,
	) -> Result<Vec<AccessibleProxy<'p>>, AtspiError> {
		let mut results = Vec::new();

		while let Some((node, depth)) = stack.pop() {
			search.deadline.check()?;

			if search.matcher.matches(&node).await? {
				results.push(node.clone());
				if search.is_satisfied_by(&results) {
					break;
				}
			}

			// Requesting the children of a node at `max_depth` would only yield nodes the
			// traversal is not allowed to visit
			if depth < self.max_depth {
				for child in self.children_of(&node).await?.into_iter().rev() {
					stack.push((child, depth + 1));
				}
			}
		}

		Ok(results)
	}

	/// Visit `nodes` in the order given, without descending into their children.
	///
	/// This is what the restricted traversal types need: they look at a single level of the
	/// tree, so `max_depth` - which is there to keep a descent from running away - does not
	/// come into play.
	async fn collect_level<'p>(
		&self,
		search: &Search,
		nodes: Vec<AccessibleProxy<'p>>,
	) -> Result<Vec<AccessibleProxy<'p>>, AtspiError> {
		let mut results = Vec::new();

		for node in nodes {
			search.deadline.check()?;

			if search.matcher.matches(&node).await? {
				results.push(node);
				if search.is_satisfied_by(&results) {
					break;
				}
			}
		}

		Ok(results)
	}

	/// Walk backwards in canonical order from `start`, collecting matches nearest-first.
	///
	/// `start` is never itself a candidate. The walk stops at `scope_root`, which is only a
	/// candidate itself if `include_scope_root` is set.
	async fn collect_preceding<'p>(
		&'p self,
		search: &Search,
		start: (&AccessibleProxy<'p>, u32),
		scope_root: &AccessibleProxy<'_>,
		include_scope_root: bool,
	) -> Result<Vec<AccessibleProxy<'p>>, AtspiError> {
		let mut results = Vec::new();
		let (mut node, mut depth) = (start.0.clone(), start.1);

		while let Some((previous, previous_depth)) =
			self.preorder_predecessor(&node, depth, scope_root).await?
		{
			search.deadline.check()?;

			if !include_scope_root && Self::is_same_object(&previous, scope_root) {
				break;
			}

			if previous_depth <= self.max_depth && search.matcher.matches(&previous).await? {
				results.push(previous.clone());
				if search.is_satisfied_by(&results) {
					break;
				}
			}

			node = previous;
			depth = previous_depth;
		}

		Ok(results)
	}

	/// Order the collected matches as `sortby` asks for.
	///
	/// `collected_canonically` tells whether the traversal encountered the matches in
	/// canonical order, which is the case for every forward walk, or in reverse canonical
	/// order, which is the case for every backward walk.
	fn apply_sort_order(
		mut results: Vec<AccessibleProxy<'_>>,
		sortby: SortOrder,
		collected_canonically: bool,
	) -> Result<Vec<AccessibleProxy<'_>>, AtspiError> {
		let canonical = match sortby {
			SortOrder::Canonical => true,
			SortOrder::ReverseCanonical => false,
			unsupported => {
				return Err(AtspiError::Owned(format!("Unsupported SortOrder: {unsupported:?}")))
			}
		};

		if canonical != collected_canonically {
			results.reverse();
		}

		Ok(results)
	}

	/// The `traverse` argument is not supported by the known serverside implementation
	/// either, so it is rejected rather than silently ignored.
	fn reject_traverse(traverse: bool) -> Result<(), AtspiError> {
		if traverse {
			return Err(AtspiError::Owned("Traverse not supported".to_string()));
		}
		Ok(())
	}
}

impl CollectionClientside for TraversalHelper<'_> {
	/// Find the closest Accessible with State:Active starting from the root object
	async fn get_active_descendant(&self) -> Result<AccessibleProxy<'_>, AtspiError> {
		let deadline = Deadline::new(self.max_time);

		// The closest active descendant is wanted, so the tree is walked breadth-first
		// rather than in canonical order
		let mut queue = VecDeque::new();
		queue.push_back((self.root.clone(), 0u32));

		while let Some((node, current_depth)) = queue.pop_front() {
			deadline.check()?;

			if current_depth > self.max_depth {
				continue;
			}

			if node.get_state().await?.contains(State::Active) {
				return Ok(node);
			}

			if current_depth < self.max_depth {
				for child in self.children_of(&node).await? {
					queue.push_back((child, current_depth + 1));
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
		Self::reject_traverse(traverse)?;
		let search = Search::new(rule, count, self.max_time)?;

		let results = self
			.collect_from_stack(&search, vec![(self.root.clone(), 0u32)])
			.await?;

		Self::apply_sort_order(results, sortby, true)
	}

	async fn get_matches_from(
		&self,
		current_object: &ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		count: i32,
		traverse: bool,
	) -> Result<Vec<AccessibleProxy<'_>>, AtspiError> {
		Self::reject_traverse(traverse)?;
		let search = Search::new(rule, count, self.max_time)?;

		let current = self.accessible_at_path(current_object).await?;

		// Every traversal type walks forwards from `current_object` here
		let results = match tree {
			TreeTraversalType::Inorder => {
				let (ancestors, current_depth) =
					self.ancestors_to_root(&current, &search.deadline).await?;
				let stack = self
					.continuation_after(&current, current_depth, &ancestors, &search.deadline)
					.await?;
				self.collect_from_stack(&search, stack).await?
			}
			TreeTraversalType::RestrictChildren => {
				let children = self.children_of(&current).await?;
				self.collect_level(&search, children).await?
			}
			TreeTraversalType::RestrictSibling => {
				let parent = self.parent_of(&current).await?;
				let (_, following) = self.siblings_around(&current, parent.as_ref()).await?;
				self.collect_level(&search, following).await?
			}
		};

		Self::apply_sort_order(results, sortby, true)
	}

	async fn get_matches_to(
		&self,
		current_object: &ObjectPath<'_>,
		rule: ObjectMatchRule,
		sortby: SortOrder,
		tree: TreeTraversalType,
		limit_scope: bool,
		count: i32,
		traverse: bool,
	) -> Result<Vec<AccessibleProxy<'_>>, AtspiError> {
		Self::reject_traverse(traverse)?;
		let search = Search::new(rule, count, self.max_time)?;

		let current = self.accessible_at_path(current_object).await?;

		// Each traversal type walks backwards from `current_object`, so that a limited
		// search yields the matches nearest to it
		let results = match tree {
			TreeTraversalType::Inorder => {
				let (ancestors, current_depth) =
					self.ancestors_to_root(&current, &search.deadline).await?;

				// With `limit_scope` set, the walk may not leave the subtree of the parent,
				// and the parent itself is not a descendant of itself, so it is excluded
				let (scope_root, include_scope_root) = match (limit_scope, ancestors.first()) {
					(false, _) => (self.root.clone(), true),
					(true, Some(parent)) => (parent.clone(), false),
					// The root has no parent to limit the scope to
					(true, None) => return Ok(Vec::new()),
				};

				self.collect_preceding(
					&search,
					(&current, current_depth),
					&scope_root,
					include_scope_root,
				)
				.await?
			}
			TreeTraversalType::RestrictChildren => {
				let mut children = self.children_of(&current).await?;
				children.reverse();
				self.collect_level(&search, children).await?
			}
			TreeTraversalType::RestrictSibling => {
				let parent = self.parent_of(&current).await?;
				let (mut preceding, _) = self.siblings_around(&current, parent.as_ref()).await?;
				preceding.reverse();
				self.collect_level(&search, preceding).await?
			}
		};

		Self::apply_sort_order(results, sortby, false)
	}
}

#[cfg(test)]
mod tests {
	use super::{ObjectMatchRuleHelper, Search};
	use atspi_common::{
		Interface, InterfaceSet, MatchType, ObjectMatchRule, Role, State, StateSet,
	};
	use std::collections::HashMap;

	fn attributes(pairs: &[(&str, &str)]) -> HashMap<String, String> {
		pairs
			.iter()
			.map(|(k, v)| ((*k).to_string(), (*v).to_string()))
			.collect()
	}

	#[test]
	fn role_match_types() {
		let matcher = |mt| {
			ObjectMatchRuleHelper::new(
				ObjectMatchRule::builder()
					.roles(&[Role::Button, Role::Link], mt)
					.build(),
			)
		};

		// An accessible has exactly one role, so `All` and `Any` are equivalent for roles
		for mt in [MatchType::All, MatchType::Any, MatchType::Empty] {
			assert!(matcher(mt).role_matches(Role::Button));
			assert!(!matcher(mt).role_matches(Role::Heading));
		}

		assert!(!matcher(MatchType::None).role_matches(Role::Button));
		assert!(matcher(MatchType::None).role_matches(Role::Heading));

		// An invalid match type means the role is not part of the query at all
		assert!(matcher(MatchType::Invalid).role_matches(Role::Heading));
	}

	#[test]
	fn empty_role_criterion() {
		let matcher =
			|mt| ObjectMatchRuleHelper::new(ObjectMatchRule::builder().roles(&[], mt).build());

		// Asking for all of no roles matches everything
		assert!(matcher(MatchType::All).role_matches(Role::Button));
		// ... but asking for any of no roles matches nothing
		assert!(!matcher(MatchType::Any).role_matches(Role::Button));
		// ... and no accessible has an empty set of roles
		assert!(!matcher(MatchType::Empty).role_matches(Role::Button));
	}

	#[test]
	fn state_match_types() {
		let matcher = |mt| {
			ObjectMatchRuleHelper::new(
				ObjectMatchRule::builder()
					.states([State::Focusable, State::Focused], mt)
					.build(),
			)
		};

		let both = StateSet::new(State::Focusable | State::Focused);
		let one = StateSet::new(State::Focusable);
		let neither = StateSet::new(State::Visible);

		assert!(matcher(MatchType::All).states_match(both));
		assert!(!matcher(MatchType::All).states_match(one));

		assert!(matcher(MatchType::Any).states_match(one));
		assert!(!matcher(MatchType::Any).states_match(neither));

		assert!(matcher(MatchType::None).states_match(neither));
		assert!(!matcher(MatchType::None).states_match(one));

		assert!(matcher(MatchType::Invalid).states_match(neither));
	}

	#[test]
	fn empty_state_criterion() {
		let matcher = |mt| {
			ObjectMatchRuleHelper::new(
				ObjectMatchRule::builder().states(Vec::<State>::new(), mt).build(),
			)
		};

		// `Empty` on an empty criterion asks the accessible's own set to be empty too
		assert!(matcher(MatchType::Empty).states_match(StateSet::empty()));
		assert!(!matcher(MatchType::Empty).states_match(StateSet::new(State::Visible)));

		assert!(matcher(MatchType::All).states_match(StateSet::new(State::Visible)));
	}

	#[test]
	fn interface_match_types() {
		let matcher = |mt| {
			ObjectMatchRuleHelper::new(
				ObjectMatchRule::builder()
					.interfaces([Interface::Text, Interface::Action], mt)
					.build(),
			)
		};

		let both = InterfaceSet::new(Interface::Text | Interface::Action);
		let one = InterfaceSet::new(Interface::Text);
		let neither = InterfaceSet::new(Interface::Value);

		assert!(matcher(MatchType::All).interfaces_match(both));
		assert!(!matcher(MatchType::All).interfaces_match(one));

		assert!(matcher(MatchType::Any).interfaces_match(one));
		assert!(!matcher(MatchType::Any).interfaces_match(neither));

		assert!(matcher(MatchType::None).interfaces_match(neither));
		assert!(!matcher(MatchType::None).interfaces_match(one));
	}

	#[test]
	fn attribute_match_types() {
		let matcher = |mt| {
			ObjectMatchRuleHelper::new(
				ObjectMatchRule::builder()
					.attributes(attributes(&[("level", "1"), ("tag", "h1")]), mt)
					.build(),
			)
		};

		let both = attributes(&[("level", "1"), ("tag", "h1")]);
		let one = attributes(&[("level", "1")]);
		// A matching key with a different value is not a match
		let wrong_value = attributes(&[("level", "2"), ("tag", "h2")]);

		assert!(matcher(MatchType::All).attributes_match(&both));
		assert!(!matcher(MatchType::All).attributes_match(&one));

		assert!(matcher(MatchType::Any).attributes_match(&one));
		assert!(!matcher(MatchType::Any).attributes_match(&wrong_value));

		assert!(matcher(MatchType::None).attributes_match(&wrong_value));
		assert!(!matcher(MatchType::None).attributes_match(&one));

		assert!(matcher(MatchType::Empty).attributes_match(&both));
	}

	#[test]
	fn inverted_rule() {
		let rule = ObjectMatchRule::builder()
			.roles(&[Role::Button], MatchType::All)
			.invert(true)
			.build();
		let matcher = ObjectMatchRuleHelper::new(rule);

		// `invert` is applied to the rule as a whole, not to the individual criteria
		assert!(matcher.role_matches(Role::Button));
	}

	#[test]
	fn count_bounds_the_search() {
		let rule = || ObjectMatchRule::builder().build();

		// The `Collection` interface uses 0 to mean "no limit"
		assert_eq!(Search::new(rule(), 0, None).unwrap().limit, None);
		assert_eq!(Search::new(rule(), 3, None).unwrap().limit, Some(3));
		assert!(Search::new(rule(), -1, None).is_err());
	}
}
