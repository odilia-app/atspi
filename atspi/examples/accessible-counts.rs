//! This example gets the root accessible object and
//! traverses the accessibility tree to get the number
//! of accessible objects for each running application
//!
//! ```sh
//! cargo run --example accessible-counts
//! ```
//! Authors:
//!    Colton Loftus

use atspi::connection::set_session_accessibility;
use atspi::proxy::accessible::ObjectRefExt;
use atspi::ObjectRefOwned;
use futures::future::try_join_all;
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;

	let root = atspi.root_accessible_on_registry().await?;

	// we have to use a hashmap to map the id to the natural
	// language name since the get_application method on the
	// accessible proxy for other items in the tree return an id
	// but not the natural language name of the associated app;
	// thus we need this to map the id to the natural language name
	let mut id_to_name = HashMap::new();
	let children = root.get_children().await?;

	// by getting the names of root's children
	// we can get the names of all currently running applications.
	for child in children.into_iter().filter_map(ObjectRefOwned::into_non_null) {
		let proxy = child.as_accessible_proxy(conn).await?;
		let natural_name = proxy.name().await?;
		let id = proxy
			.get_application()
			.await?
			.name()
			.expect("root object has a name")
			.to_string();
		id_to_name.insert(id, natural_name);
	}

	// this stack represents all Accessible objects that
	// have not yet been processed; it is used for the DFS traversal
	let mut tmp_stack = vec![root];

	let mut id_to_accessible_count = HashMap::new();

	println!("Traversing tree...");
	while let Some(ap) = tmp_stack.pop() {
		let child_objects = ap.get_children().await?;
		let children_proxies = try_join_all(
			child_objects
				.into_iter()
				.filter_map(ObjectRefOwned::into_non_null)
				.map(|child| child.into_accessible_proxy(conn)),
		)
		.await?;

		for child in &children_proxies {
			let application_name = child
				.get_application()
				.await?
				.name()
				.expect("root object has a bus name")
				.to_string();
			*id_to_accessible_count.entry(application_name).or_insert(0) += 1;
		}
		tmp_stack.extend(children_proxies);
	}

	for (id, count) in id_to_accessible_count {
		match id_to_name.get(&id) {
			Some(name) => println!("Application '{name}' has {count} accessible objects"),
			None => eprintln!(
				"Couldn't get name for app id '{id}' with {count} associated accessible objects"
			),
		}
	}
	Ok(())
}
