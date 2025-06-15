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
		// Contains the processed `A11yNode`'s.
		let mut nodes: Vec<A11yNode> = Vec::new();

		// Contains the `AccessibleProxy` yet to be processed.
		let mut stack: Vec<AccessibleProxy> = vec![ap];

		// If the stack has an `AccessibleProxy`, we take the last.
		while let Some(ap) = stack.pop() {
			let name = ap.name().await;
			let bus_name = ap.inner().destination();

			let node_name = {
				match name {
					Ok(name) => format!("node: {name} on {bus_name}"),
					Err(e) => {
						eprintln!(
							"Error getting name for {}: {e} -- continuing with next node.",
							ap.inner().path()
						);
						format!("node: \"Unknown name\" on {bus_name}")
					}
				}
			};

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

			let child_count = child_objects.len();
			if child_count > 65536 {
				eprintln!("Error: Child count on {node_name} exceeds 65536, (has {child_count}).");
				return Err("Child count exceeds limit".into());
			}

			if child_objects.is_empty() {
				// If there are no children, we can get the role and continue.
				let role = ap.get_role().await.ok();

				// Create a node with the role and no children.
				nodes.push(A11yNode { role, children: Vec::new() });
				continue;
			}

			let mut children_proxies: Vec<AccessibleProxy> = Vec::with_capacity(child_count);
			for child in child_objects.into_iter() {
				match a11y.object_as_accessible(&child).await {
					Ok(proxy) => children_proxies.push(proxy),
					Err(_err) => {
						#[cfg(feature = "tracing")]
						tracing::debug!(
							"Failed to get accessible proxy for child: {}",
							&child.name
						);
					}
				}
			}

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
	println!("Tree built in {elapsed:.2?} using P2P connections.");

	Ok(())
}
