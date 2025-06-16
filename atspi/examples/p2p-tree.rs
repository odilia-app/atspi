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
	AccessibilityConnection, Role,
};
use atspi_connection::P2P;
use futures::future::join_all;
use std::fmt::{self, Display, Formatter};
use zbus::{proxy::CacheProperties, Connection};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const REGISTRY_DEST: &str = "org.a11y.atspi.Registry";
const ROOT_ACCESSIBLE_PATH: &str = "/org/a11y/atspi/accessible/root";

#[derive(Debug, PartialEq, Eq, Clone)]
struct A11yNode {
	role: Option<Role>,
	children: Vec<A11yNode>,
}

#[derive(Clone, Copy)]
pub struct CharSet {
	pub horizontal: char,
	pub vertical: char,
	pub connector: char,
	pub end_connector: char,
}
pub const SINGLE_LINE: CharSet =
	CharSet { horizontal: '─', vertical: '│', connector: '├', end_connector: '└' };

impl Display for A11yNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.fmt_with(f, SINGLE_LINE, &mut Vec::new())
	}
}

impl A11yNode {
	fn fmt_with(
		&self,
		f: &mut std::fmt::Formatter<'_>,
		style: CharSet,
		prefix: &mut Vec<bool>,
	) -> std::fmt::Result {
		for (i, is_last_at_i) in prefix.iter().enumerate() {
			// if it is the last portion of the line
			let is_last = i == prefix.len() - 1;
			match (is_last, *is_last_at_i) {
				(true, true) => write!(f, "{}", style.end_connector)?,
				(true, false) => write!(f, "{}", style.connector)?,
				// four spaces to emulate `tree`
				(false, true) => write!(f, "    ")?,
				// three spaces and vertical char
				(false, false) => write!(f, "{}   ", style.vertical)?,
			}
		}

		// two horizontal chars to mimic `tree`
		let role_string = self
			.role
			.map_or_else(|| "error".to_string(), |r| r.to_string())
			.to_string();
		writeln!(f, "{}{} {}", style.horizontal, style.horizontal, role_string)?;

		for (i, child) in self.children.iter().enumerate() {
			prefix.push(i == self.children.len() - 1);
			child.fmt_with(f, style, prefix)?;
			prefix.pop();
		}

		Ok(())
	}
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

async fn get_registry_accessible<'a>(conn: &Connection) -> Result<AccessibleProxy<'a>> {
	let registry = AccessibleProxy::builder(conn)
		.destination(REGISTRY_DEST)?
		.path(ROOT_ACCESSIBLE_PATH)?
		.cache_properties(CacheProperties::No)
		.build()
		.await?;

	Ok(registry)
}

#[tokio::main]
async fn main() -> Result<()> {
	set_session_accessibility(true).await?;

	let a11y = AccessibilityConnection::new().await?;
	let conn = a11y.connection();
	let registry = get_registry_accessible(conn).await?;

	let no_children = registry.child_count().await?;
	println!("Number of accessible applications on the a11y-bus: {no_children}");

	println!("Building tree (P2P)...");
	let now = std::time::Instant::now();
	let _tree = A11yNode::from_accessible_proxy(registry.clone(), &a11y).await?;
	let elapsed = now.elapsed();

	println!("Built tree with {} nodes in {:.2?}", _tree.node_count(), elapsed);

	Ok(())
}
