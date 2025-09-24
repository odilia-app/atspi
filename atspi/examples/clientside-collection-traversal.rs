//! This example gets the root accessible object and
//! traverses the accessibility tree to get the number
//! of accessible objects for each running application
//!
//! ```sh
//! cargo run --example clientside-collection-traversal
//! ```
//! Authors:
//!    Colton Loftus
use atspi::connection::set_session_accessibility;
use atspi::proxy::accessible::ObjectRefExt;
use atspi_proxies::traversal_helper::{CollectionClientside, TraversalHelper};
use std::error::Error;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
	set_session_accessibility(true).await?;
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();

	let root = atspi.root_accessible_on_registry().await?;

	const DEPTH: u32 = 10;

	// we can get the names of all applications currently running
	for child in root.get_children().await?.iter() {
		let traversal_helper = TraversalHelper::new(
			child.to_owned().into_accessible_proxy(conn).await?,
			conn.clone(),
			DEPTH,
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
