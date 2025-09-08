//! This example demonstrates the advantage of P2P connections can offer
//!
//! ```sh
//! cargo run --example p2p-tree
//! ```
//! Authors:
//!    Luuk van der Duim,
//!    Tait Hoyem

use atspi::{
	connection::set_session_accessibility, proxy::accessible::AccessibleProxy,
	AccessibilityConnection, NonNullObjectRef, Role,
};
use atspi_connection::P2P;
use atspi_proxies::accessible::ObjectRefExt;
use futures::{
	future::{join_all, try_join_all},
	stream::FuturesUnordered,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct A11yNode {
	role: Option<Role>,
	children: Vec<A11yNode>,
}

impl A11yNode {
	async fn from_accessible_proxy(
		ap: AccessibleProxy<'_>,
		a11y: &AccessibilityConnection,
	) -> Result<A11yNode> {
		// The nodes list gets populated with two kinds of nodes:
		// 1. Nodes with children, which have no children of themselves yet.
		// 2. Leaf nodes, which have no children.
		let mut nodes: Vec<A11yNode> = Vec::new();

		// Contains the `AccessibleProxy`s.
		// Contains the source for the next iterations of work.
		// Initialized with the root `AccessibleProxy`.
		let mut stack: Vec<AccessibleProxy> = vec![ap];

		// If the stack has an `AccessibleProxy`, we take the last.
		while let Some(ap) = stack.pop() {
			let bus_name = ap.inner().destination();
			let node_name = match ap.name().await {
				Ok(name) => format!("node: {name} on {bus_name}"),
				Err(_) => format!("node: \"Unknown name\" on {bus_name}"),
			};

			let role = ap.get_role().await.ok();

			match ap.get_children().await {
				Err(e) => {
					eprintln!("Error obtaining children: {node_name}: {e} - continuing.");

					// You would probably want to encode in the `A11yNode` that this node has an error condition with its children.
					// For this example, we just create a node without children.

					nodes.push(A11yNode { role, children: Vec::new() });
				}

				Ok(children) if children.is_empty() => {
					nodes.push(A11yNode { role, children: Vec::new() });
				}

				Ok(children) if children.len() > 65536 => {
					eprintln!("Warning: {node_name} exceeds 65536 children, creating empty node.");

					// If the number of children exceeds 65536:
					// create a node without children.
					// (We need to create a node because we might be a child of an earlier node.)

					// Note: One could also design `A11yNode` to encode this error condition.

					nodes.push(A11yNode { role, children: Vec::new() });
				}

				Ok(children) => {
					let mut children_proxies: Vec<AccessibleProxy> =
						Vec::with_capacity(children.len());

					for child in children.into_iter() {
						let Ok(child) = NonNullObjectRef::try_from(child) else { continue };

						match a11y.object_as_accessible(&child).await {
							Ok(proxy) => children_proxies.push(proxy),
							Err(e) => {
								// A problem with the `trait P2P` method `object_as_accessible`
								// we should be able to create an `AccessibleProxy` for this object.
								return Err(format!(
									"Error creating AccessibleProxy for {node_name}: {e}"
								)
								.into());
							}
						}
					}

					// We create as-many role _results_ as there are child proxies.
					let role_results =
						join_all(children_proxies.iter().map(|child| child.get_role())).await;

					debug_assert_eq!(
						role_results.len(),
						children_proxies.len(),
						"Role results length does not match children proxies length"
					);

					stack.append(&mut children_proxies);

					let children = role_results
						.into_iter()
						.map(|role| A11yNode { role: role.ok(), children: Vec::new() })
						.collect::<Vec<_>>();

					nodes.push(A11yNode { role, children });
				}
			};
		}

		// The nodes list now gets 'unwound' LIFO order.
		// The first nodes popped are leaves, these are pushed onto the `fold_stack`.
		// When we encounter a node with children, we pop the required number of nodes from the `fold_stack`
		// and assign them as children to the node.
		// This way we build the tree from the bottom up, folding in the leaf nodes as we go.

		let mut fold_stack: Vec<A11yNode> = Vec::with_capacity(nodes.len());

		while let Some(mut node) = nodes.pop() {
			if node.children.is_empty() {
				fold_stack.push(node);
				continue;
			}

			// If the node has children, we fold in the children from 'fold_stack'.
			// There may be more on 'fold_stack' than the node requires.
			let begin = fold_stack.len().saturating_sub(node.children.len());
			node.children = fold_stack.split_off(begin);
			fold_stack.push(node);
		}

		fold_stack.pop().ok_or("No root node built".into())
	}

	fn node_count(&self) -> usize {
		let mut count = 1; // Count this node
		for child in &self.children {
			count += child.node_count();
		}
		count
	}
}

impl A11yNode {
	async fn from_accessible_proxy_bus(ap: AccessibleProxy<'_>) -> Result<A11yNode> {
		let connection = ap.inner().connection().clone();
		// Contains the processed `A11yNode`'s.
		let mut nodes: Vec<A11yNode> = Vec::new();

		// Contains the `AccessibleProxy` yet to be processed.
		let mut stack: Vec<AccessibleProxy> = vec![ap];

		// If the stack has an `AccessibleProxy`, we take the last.
		while let Some(ap) = stack.pop() {
			let destination = ap.inner().destination();
			let mut node_name = format!("node: Unknown node on {destination}");
			if let Ok(name) = ap.name().await {
				node_name = format!("node: {name} on {destination}");
			}

			let child_objects = ap.get_children().await;
			let child_objects = match child_objects {
				// Ok can also be an empty vector, which is fine.
				Ok(children) => children,
				Err(e) => {
					eprintln!(
						"Error getting children of {node_name}: {e} -- continuing with next node."
					);
					continue;
				}
			};

			if child_objects.is_empty() {
				// If there are no children, we can get the role and continue.
				let role = ap.get_role().await.ok();

				// Create a node with the role and no children.
				nodes.push(A11yNode { role, children: Vec::new() });
				continue;
			}

			// Very likely to succeed because the error can only happen if the property cache is enabled,
			// which we disable in `into_accessible_proxy`.
			let mut children_proxies = try_join_all(
				child_objects
					.into_iter()
					.filter_map(|child| NonNullObjectRef::try_from(child).ok()) // Filter out null and convert
					.map(|child| child.into_accessible_proxy(&connection)),
			)
			.await?;

			let roles = join_all(children_proxies.iter().map(|child| child.get_role())).await;
			stack.append(&mut children_proxies);
			// Now we have the role results of the child nodes, we can create `A11yNode`s for them.
			let children = roles
				.into_iter()
				.map(|role| A11yNode { role: role.ok(), children: Vec::new() })
				.collect::<Vec<_>>();

			// Finaly get this node's role and create an `A11yNode` with it.
			let role = ap.get_role().await.ok();
			nodes.push(A11yNode { role, children });
		}

		let mut fold_stack: Vec<A11yNode> = Vec::with_capacity(nodes.len());

		while let Some(mut node) = nodes.pop() {
			if node.children.is_empty() {
				fold_stack.push(node);
				continue;
			}

			// If the node has children, we fold in the children from 'fold_stack'.
			// There may be more on 'fold_stack' than the node requires.
			let begin = fold_stack.len().saturating_sub(node.children.len());
			node.children = fold_stack.split_off(begin);
			fold_stack.push(node);
		}

		fold_stack.pop().ok_or("No root node built".into())
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	set_session_accessibility(true).await?;

	let a11y = AccessibilityConnection::new().await?;
	let registry = a11y.root_accessible_on_registry().await?;

	let child_count = registry.child_count().await?;
	println!("Number of accessible applications on the a11y-bus: {child_count}");

	// Define fixed widths for the description, node count and time columns
	const DESC_WIDTH: usize = 30;
	const NODE_COUNT_WIDTH: usize = 10;
	const TIME_WIDTH: usize = 10;

	println!();
	let table_header = format!(
		"{:<DESC_WIDTH$} {:<NODE_COUNT_WIDTH$} {:<TIME_WIDTH$}",
		"D-Bus operation", "Node count", "Time (ms)"
	);
	let table_divider = "-".repeat(DESC_WIDTH + NODE_COUNT_WIDTH + TIME_WIDTH + 2).to_string();

	print!("Building tree using the bus... ");
	let now = std::time::Instant::now();
	let _tree_bus = A11yNode::from_accessible_proxy_bus(registry.clone()).await?;
	let bus_elapsed = now.elapsed();
	println!(" done.");
	let bus_tree_line = format!(
		"{:<DESC_WIDTH$} {:<NODE_COUNT_WIDTH$} {:<TIME_WIDTH$.2?}",
		"Building tree (bus)",
		_tree_bus.node_count(), // Count of nodes in the tree
		bus_elapsed.as_secs_f64() * 1000.0
	);

	print!("Building tree using P2P...");
	let now = std::time::Instant::now();
	let _tree_p2p = A11yNode::from_accessible_proxy(registry.clone(), &a11y).await?;
	let p2p_elapsed = now.elapsed();
	println!(" done.");
	let p2p_tree_line = format!(
		"{:<DESC_WIDTH$} {:<NODE_COUNT_WIDTH$} {:<TIME_WIDTH$.2?}",
		"Building tree (P2P)",
		_tree_p2p.node_count(),
		p2p_elapsed.as_secs_f64() * 1000.0
	);

	print!("Building tree using P2P (parallel)...");
	let now = std::time::Instant::now();
	let registry_role = registry.get_role().await.ok();
	let bus_applications = registry.get_children().await?;

	let futures = bus_applications
		.into_iter()
		.map(|child| {
			let a11y_clone = a11y.clone();

			async move {
				let non_null_child =
					NonNullObjectRef::try_from(child).expect("Child should be NonNullObjectRef");
				let proxy = a11y_clone.object_as_accessible(&non_null_child).await?;
				A11yNode::from_accessible_proxy(proxy, &a11y_clone).await
			}
		})
		.collect::<Vec<_>>();

	let mut applications_unordered = FuturesUnordered::from_iter(futures);
	let mut children = Vec::new();
	while let Some(node) = futures::StreamExt::next(&mut applications_unordered).await {
		match node {
			Ok(node) => children.push(node),
			Err(e) => eprintln!("Error building node: {e}"),
		}
	}
	let _tree_par = A11yNode { role: registry_role, children };
	let p2p_par_elapsed = now.elapsed();
	println!(" done.\n\n");

	let p2p_par_line = format!(
		"{:<DESC_WIDTH$} {:<NODE_COUNT_WIDTH$} {:<TIME_WIDTH$.2?}",
		"Building tree (P2P parallel)",
		_tree_par.node_count(),
		p2p_par_elapsed.as_secs_f64() * 1000.0
	);

	// Print the table header
	println!("{table_header}");
	println!("{table_divider}");
	// Print the table rows
	println!("{bus_tree_line}");
	println!("{p2p_tree_line}");
	println!("{p2p_par_line}");
	// Print the table divider again
	println!("{table_divider}");

	// Print speedup with bus as baseline
	let bus_speedup = bus_elapsed.as_secs_f64() / p2p_elapsed.as_secs_f64();
	let par_speedup = bus_elapsed.as_secs_f64() / p2p_par_elapsed.as_secs_f64();
	println!("{:<DESC_WIDTH$} {:.2}x", "Speedup (p2p vs bus)", bus_speedup);
	println!("{:<DESC_WIDTH$} {:.2}x", "Speedup (p2p-par vs bus)", par_speedup);

	Ok(())
}
