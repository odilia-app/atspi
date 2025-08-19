//! This example demonstrates how to construct a tree of accessible objects on the accessibility-bus.
//!
//! "This example requires the  `proxies-tokio`, `tokio` and `zbus` features to be enabled:
//!
//! ```sh
//! cargo run --example bus-tree --features zbus,proxies-tokio,tokio
//! ```
//! Authors:
//!    Luuk van der Duim,
//!    Tait Hoyem

use atspi::{
	connection::set_session_accessibility,
	proxy::accessible::{AccessibleProxy, ObjectRefExt},
	AccessibilityConnection, Role,
};
use futures::future::{join_all, try_join_all};
use std::fmt::{self, Display, Formatter};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
	async fn from_accessible_proxy(ap: AccessibleProxy<'_>) -> Result<A11yNode> {
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
					.filter(|child| !child.is_null()) // Filter out null children
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

	let no_children = registry.child_count().await?;
	println!("Number of accessible applications on the a11y-bus: {no_children}");
	println!("Construct a tree of accessible objects on the a11y-bus\n");

	let now = std::time::Instant::now();
	let tree = A11yNode::from_accessible_proxy(registry).await?;
	let elapsed = now.elapsed();
	println!("Elapsed time: {elapsed:.2?}");

	println!("\nPress 'Enter' to print the tree...");
	let _ = std::io::stdin().read_line(&mut String::new());

	println!("{tree}");

	Ok(())
}
