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
use atspi::proxy::accessible::{AccessibleProxy, ObjectRefExt};
use futures::future::try_join_all;
use std::collections::{hash_map, HashMap};
use std::error::Error;
use zbus::proxy::CacheProperties;
use zbus::Connection;

// We use the top level atspi Registry to get the root of
// the accessiblity tree of the desktop
const REGISTRY_DEST: &str = "org.a11y.atspi.Registry";
const REGISTRY_PATH: &str = "/org/a11y/atspi/accessible/root";
const ACCCESSIBLE_INTERFACE: &str = "org.a11y.atspi.Accessible";

async fn get_registry_accessible<'a>(
	conn: &Connection,
) -> Result<AccessibleProxy<'a>, Box<dyn Error>> {
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
async fn main() -> std::result::Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;

	let root = get_registry_accessible(conn).await?;

	// we have to use a hashmap to map the id to the natural
	// language name since the get_application method on the
	// accessible proxy for other items in the tree return an id
	// but not the natural language name of the associated app;
	// thus we need this to map the id to the natural language name
	let mut id_to_name = HashMap::new();

	// by getting the names of the children of the root
	// we can get the names of all applications currently running
	for child in root.get_children().await?.iter() {
		let proxy = child.clone().into_accessible_proxy(conn).await?;
		let natural_name = proxy.name().await?;
		let id = proxy.get_application().await?.name.to_string();
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
				.map(|child| child.into_accessible_proxy(conn)),
		)
		.await?;
		for child in &children_proxies {
			let application_name = child.get_application().await?.name.to_string();
			match id_to_accessible_count.entry(application_name) {
				hash_map::Entry::Vacant(e) => {
					e.insert(1);
				}
				hash_map::Entry::Occupied(mut e) => {
					let count = e.get_mut();
					*count += 1;
				}
			}
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
