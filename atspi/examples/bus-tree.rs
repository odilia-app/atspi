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
use display_tree::{AsTree, DisplayTree, Style};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const REGISTRY_DEST: &str = "org.a11y.atspi.Registry";
const REGISTRY_PATH: &str = "/org/a11y/atspi/accessible/root";
const ACCCESSIBLE_INTERFACE: &str = "org.a11y.atspi.Accessible";

#[derive(Debug)]
struct A11yNode {
	role: Role,
	children: Vec<A11yNode>,
}

impl DisplayTree for A11yNode {
	fn fmt(&self, f: &mut std::fmt::Formatter, style: Style) -> std::fmt::Result {
		self.fmt_with(f, style, &mut vec![])
	}
}

impl A11yNode {
	fn fmt_with(
		&self,
		f: &mut std::fmt::Formatter<'_>,
		style: Style,
		prefix: &mut Vec<bool>,
	) -> std::fmt::Result {
		for (i, is_last_at_i) in prefix.iter().enumerate() {
			// if it is the last portion of the line
			let is_last = i == prefix.len() - 1;
			match (is_last, *is_last_at_i) {
				(true, true) => write!(f, "{}", style.char_set.end_connector)?,
				(true, false) => write!(f, "{}", style.char_set.connector)?,
				// four spaces to emulate `tree`
				(false, true) => write!(f, "    ")?,
				// three spaces and vertical char
				(false, false) => write!(f, "{}   ", style.char_set.vertical)?,
			}
		}

		// two horizontal chars to mimic `tree`
		writeln!(f, "{}{} {}", style.char_set.horizontal, style.char_set.horizontal, self.role)?;

		for (i, child) in self.children.iter().enumerate() {
			prefix.push(i == self.children.len() - 1);
			child.fmt_with(f, style, prefix)?;
			prefix.pop();
		}

		Ok(())
	}
}

impl A11yNode {
	async fn from_accessible_proxy(ap: AccessibleProxy<'_>) -> Result<Self> {
		let role = ap.get_role().await?;
		let child_objs = ap.get_children().await?;
		let connection = ap.inner().connection();

		// Convert `Vec<ObjectRef>` to a `Vec<Future<Output = AccessibleProxy>`.
		let children = child_objs
			.iter()
			.map(|child| child.as_accessible_proxy(connection))
			.collect::<Vec<_>>();

		// Resolve the futures and filter out the errors.
		let children = futures::future::join_all(children)
			.await
			.into_iter()
			.filter_map(|child| child.ok())
			.collect::<Vec<_>>();

		// Convert to a `Vec<Future<Output = Result<A11yNode>>`.
		let children = children
			.into_iter()
			.map(|child| Box::pin(Self::from_accessible_proxy(child)))
			.collect::<Vec<_>>();

		// Resolve the futures and filter out the errors.
		let children = futures::future::join_all(children)
			.await
			.into_iter()
			.filter_map(|child| child.ok())
			.collect::<Vec<_>>();

		Ok(A11yNode { role, children })
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

	println!("{}", AsTree::new(&tree));

	Ok(())
}
