//! This example gets the root accessible object and
//! traverses the accessibility tree to get the number
//! of accessible objects for each running application
//!
//! ```sh
//! cargo run --example clientside-collection-traversal
//! ```
//! Authors:
//!    Colton Loftus
use atspi::proxy::accessible::ObjectRefExt;
use atspi::{connection::set_session_accessibility, ObjectMatchRule};
use atspi::{MatchType, Role};
use atspi_proxies::{
	accessible::AccessibleProxy,
	traversal_helper::{CollectionClientside, TraversalHelper},
};
use std::error::Error;

async fn get_active_descendant(
	root: &AccessibleProxy<'_>,
	conn: &zbus::Connection,
) -> Result<(), Box<dyn Error>> {
	const MAX_DEPTH: u32 = 10;
	// we can get the names of all applications currently running
	for child in root.get_children().await?.iter() {
		let traversal_helper = TraversalHelper::new(
			child.to_owned().into_accessible_proxy(conn).await?,
			conn.clone(),
			MAX_DEPTH,
			None,
		);
		let result = traversal_helper.get_active_descendant().await;
		if let Ok(proxy) = result {
			println!(
				"Got active descendant {} for app '{}'",
				proxy.name().await?,
				child.name().unwrap().as_str()
			);
		}
	}
	Ok(())
}

async fn get_matches(
	root: &AccessibleProxy<'_>,
	conn: &zbus::Connection,
) -> Result<(), Box<dyn Error>> {
	const MAX_DEPTH: u32 = 10;
	// we can get the names of all applications currently running
	for child in root.get_children().await?.iter() {
		let traversal_helper = TraversalHelper::new(
			child.to_owned().into_accessible_proxy(conn).await?,
			conn.clone(),
			MAX_DEPTH,
			None,
		);
		let rule = ObjectMatchRule::builder().roles(&[Role::DocumentWeb], MatchType::All);
		let result = traversal_helper
			.get_matches(rule.build(), atspi::SortOrder::Canonical, 10, false)
			.await;
		if let Ok(matches) = result {
			println!(
				"Got {} matches for accessible '{}' with the first match having name '{}'",
				matches.len(),
				child.name().unwrap().as_str(),
				match matches.first() {
					Some(proxy) => proxy.name().await?,
					_ => "None".to_string(),
				}
			);
		}
	}
	Ok(())
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
	set_session_accessibility(true).await?;
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();

	let root = atspi.root_accessible_on_registry().await?;

	println!("Getting information on the active descendant of the root object");
	get_active_descendant(&root, conn).await?;

	println!("Getting information on the matches of the root object");
	get_matches(&root, conn).await?;

	Ok(())
}
