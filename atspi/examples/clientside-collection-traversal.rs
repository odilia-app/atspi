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
use atspi::{MatchType, Role, TreeTraversalType};
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

/// Walk forwards and backwards from a single object, in the same way a screen reader
/// jumps to the next or previous element of a given role.
async fn get_matches_from_and_to(
	root: &AccessibleProxy<'_>,
	conn: &zbus::Connection,
) -> Result<(), Box<dyn Error>> {
	const MAX_DEPTH: u32 = 10;

	// The rule is consumed by each call, so it is rebuilt every time
	let buttons = || {
		ObjectMatchRule::builder()
			.roles(&[Role::Button], MatchType::All)
			.build()
	};

	for child in root.get_children().await?.iter() {
		let traversal_helper = TraversalHelper::new(
			child.to_owned().into_accessible_proxy(conn).await?,
			conn.clone(),
			MAX_DEPTH,
			None,
		);

		// Find every button of the application, then start walking from one in the middle
		// so that there is something to find in both directions
		let all = traversal_helper
			.get_matches(buttons(), atspi::SortOrder::Canonical, 0, false)
			.await?;
		if all.len() < 3 {
			continue;
		}
		let current = &all[all.len() / 2];
		let current_path = current.inner().path().to_owned();

		// The two buttons that follow it in the tree ...
		let following = traversal_helper
			.get_matches_from(
				&current_path,
				buttons(),
				atspi::SortOrder::Canonical,
				TreeTraversalType::Inorder,
				2,
				false,
			)
			.await?;

		// ... and the two that precede it, nearest first
		let preceding = traversal_helper
			.get_matches_to(
				&current_path,
				buttons(),
				atspi::SortOrder::ReverseCanonical,
				TreeTraversalType::Inorder,
				false,
				2,
				false,
			)
			.await?;

		println!(
			"Application '{}' has {} buttons; starting from '{}':",
			child.name().unwrap().as_str(),
			all.len(),
			current.name().await?
		);
		for button in preceding {
			println!("  before: '{}'", button.name().await?);
		}
		for button in following {
			println!("  after:  '{}'", button.name().await?);
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

	println!("Walking to the matches before and after a given object");
	get_matches_from_and_to(&root, conn).await?;

	Ok(())
}
