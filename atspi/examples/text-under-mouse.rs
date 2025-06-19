//! This example demonstrates how to get the currently selected text.
//!
//! ```sh
//! cargo run --example text-under-mouse
//! ```
//! Authors:
//!    Colton Loftus

use atspi::MouseEvents;
use atspi_connection::set_session_accessibility;
use atspi_proxies::{accessible::ObjectRefExt, proxy_ext::ProxyExt};
use futures_lite::stream::StreamExt;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;
	atspi.register_event::<MouseEvents>().await?;

	let mut events = atspi.event_stream();

	while let Some(ev) = events.next().await {
		let ev = match ev {
			Ok(ev) => ev,
			Err(err) => {
				eprintln!("Error: {err}");
				continue;
			}
		};

		let mouse_ev = match ev {
			atspi::Event::Mouse(ev) => ev,
			_ => continue,
		};

		let mouse_abs_ev = match mouse_ev {
			atspi::MouseEvents::Abs(mouse_ev) => mouse_ev,
			_ => continue,
		};

		let proxy = mouse_abs_ev.item.into_accessible_proxy(conn).await?;

		let proxy_result = proxy.proxies().await;

		let component = match proxy_result {
			Ok(comp) => comp.component().await?,
			Err(err) => {
				eprintln!("Error: {err}");
				continue;
			}
		};

		let accessible = component
			.get_accessible_at_point(mouse_abs_ev.x, mouse_abs_ev.y, atspi::CoordType::Screen)
			.await?;

		let text_proxy_result = accessible.into_accessible_proxy(conn).await?.proxies().await;

		let associated_text_proxy = match text_proxy_result {
			Ok(proxy) => proxy.text().await?,
			Err(err) => {
				eprintln!("Error: {err}");
				continue;
			}
		};

		let offset = associated_text_proxy
			.get_offset_at_point(mouse_abs_ev.x, mouse_abs_ev.y, atspi::CoordType::Screen)
			.await?;

		let text = associated_text_proxy.get_text(0, offset).await?;

		println!("{text}");
	}
	Ok(())
}
