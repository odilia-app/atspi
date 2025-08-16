//! This example demonstrates how to get the currently selected text.
//!
//! ```sh
//! cargo run --example selected-text
//! ```
//! Authors:
//!    Colton Loftus

use atspi::{events::object::TextSelectionChangedEvent, ObjectEvents};
use atspi_proxies::{accessible::ObjectRefExt, proxy_ext::ProxyExt};
use futures_lite::stream::StreamExt;
use std::error::Error;

// When using the text proxy, it is possible to
// get the selected text from multiple different
// ranges independent of each other. In this example
// for the sake of simplicity, we only get the first
const ASSUME_ONLY_ONE_SELECTED_RANGE: i32 = 0;

smol_macros::main! {
	async fn main() -> Result<(), Box<dyn Error>> {
		let atspi = atspi::AccessibilityConnection::new().await?;
		let conn = atspi.connection();
		atspi.register_event::<ObjectEvents>().await?;

		let mut events = atspi.event_stream();

		while let Some(ev) = events.next().await {
			match ev {
				Ok(ev) => {
					if let Ok(ev) = <TextSelectionChangedEvent>::try_from(ev) {
						let text_proxy = ev
							.item
							.into_accessible_proxy(conn)
							.await?
							.proxies()
							.await?
							.text()
							.await?;
						let (start, end) =
							text_proxy.get_selection(ASSUME_ONLY_ONE_SELECTED_RANGE).await?;
						println!("{}", text_proxy.get_text(start, end).await?);
					}
				}
				Err(err) => eprintln!("Error: {err}"),
			}
		}
		Ok(())
	}
}
