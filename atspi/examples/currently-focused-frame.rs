//! This example prints out the currently focused frame. A
//! frame is generally semantically equivalent to an application
//! window.
//!
//! ```sh
//! cargo run --example currently-focused-frame
//! ```
//! Authors:
//!    Colton Loftus

use std::error::Error;

use atspi::{ObjectRefOwned, State};
use atspi_connection::set_session_accessibility;
use atspi_proxies::accessible::ObjectRefExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;

	let apps = atspi.root_accessible_on_registry().await?.get_children().await?;

	let mut found_active_frame: bool = false;

	for app in apps.into_iter().filter_map(ObjectRefOwned::into_non_null) {
		let proxy = app.as_accessible_proxy(conn).await?;
		let state = proxy.get_state().await?;
		assert!(!state.contains(State::Active), "The top level application should never have active state; only its associated frames should have this state");

		let children = proxy.get_children().await?;
		for frame in children.into_iter().filter_map(ObjectRefOwned::into_non_null) {
			let frame = frame.as_accessible_proxy(conn).await?;
			let state = frame.get_state().await?;
			if state.contains(State::Active) {
				print!("Found active frame with title: '{}'", frame.name().await?);
				found_active_frame = true;
			}
		}
	}

	assert!(found_active_frame, "There must be one active frame at any given time");

	Ok(())
}
