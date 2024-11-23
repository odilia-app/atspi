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
	zbus::{proxy::CacheProperties, Connection},
	AccessibilityConnection, Role,
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const REGISTRY_DEST: &str = "org.a11y.atspi.Registry";
const REGISTRY_PATH: &str = "/org/a11y/atspi/accessible/root";
const ACCCESSIBLE_INTERFACE: &str = "org.a11y.atspi.Accessible";

#[derive(Debug, Serialize, Deserialize)]
struct A11yNode {
	role: Role,
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
		let mut numof = 0;
		let mut max_depth = 0;
		let mut leafs = 0;
		let mut stack: Vec<(&Self, usize, usize)> = vec![(self, 0, 0)];
		while let Some((this, siblings, idx)) = stack.pop() {
			if siblings > 0 {
				prefix.push(idx == siblings - 1);
			}
			numof += 1;
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
			writeln!(
				f,
				"{}{} {}({})",
				style.horizontal,
				style.horizontal,
				this.role,
				this.children.len()
			)?;

			for (i, child) in this.children.iter().enumerate() {
				stack.push((child, this.children.len(), i));
			}
			if this.children.len() > 0 {
				max_depth += 1;
				continue;
			} else {
				leafs += 1;
			}
			prefix.pop();
		}
		Ok(())
	}
}

impl A11yNode {
	async fn from_accessible_proxy(ap: AccessibleProxy<'_>) -> Result<Self> {
		let connection = ap.inner().connection().clone();
		let mut num_found = 0;
		let mut out = std::io::stdout();
		// Contains the processed `A11yNode`'s.
		let mut nodes: Vec<A11yNode> = Vec::new();

		// Contains the `AccessibleProxy` yet to be processed.
		let mut stack: Vec<AccessibleProxy> = vec![ap];

		// If the stack has an `AccessibleProxy`, we take the last.
		while let Some(ap) = stack.pop() {
			num_found += 1;
			if num_found % 10_000 == 0 {
				println!("Processed {num_found} element");
				let _ = out.flush();
			}
			let cc = ap.child_count().await?;
			// Prevent obects with huge child counts from stalling the program.
			if cc > 65536 {
				continue;
			}

			let child_objects = ap.get_children().await?;
			let mut children_proxies = Vec::new();
			let mut roles = Vec::new();
			for child_object in child_objects {
				let co = child_object.into_accessible_proxy(&connection).await?;
				roles.push(co.get_role().await?);
				children_proxies.push(co);
			}

			stack.append(&mut children_proxies);

			let children = roles
				.into_iter()
				.map(|role| A11yNode { role, children: Vec::new() })
				.collect::<Vec<_>>();

			let role = ap.get_role().await?;
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
			let new_children = fold_stack.split_off(begin);
			node.children = new_children;
			fold_stack.push(node);
		}

		fold_stack.pop().ok_or("No root node built".into())
	}
}

async fn get_registry_accessible<'a>(conn: &Connection) -> Result<AccessibleProxy<'a>> {
	let registry = AccessibleProxy::builder(conn)
		.destination(REGISTRY_DEST)?
		.path(REGISTRY_PATH)?
		.interface(ACCCESSIBLE_INTERFACE)?
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
	println!("Construct a tree of accessible objects on the a11y-bus\n");

	let now = std::time::Instant::now();
	let tree = A11yNode::from_accessible_proxy(registry).await?;
	let elapsed = now.elapsed();
	println!("Elapsed time: {:?}", elapsed);

	println!("\nPress 'Enter' to print the tree...");
	let _ = std::io::stdin().read_line(&mut String::new());

	println!("{}", serde_json::to_string(&tree).expect("JSON Out"));

	Ok(())
}
