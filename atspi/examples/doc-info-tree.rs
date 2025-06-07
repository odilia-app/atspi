//! This example watches for document load events and prints a tree of accessible objects
//! descending from the document root. It displays some generally useful info like
//! the name, role, and coordinates of each accessible object.
//!
//! ```sh
//! cargo run --example doc-info-tree
//! ```
//! Authors:
//!    Colton Loftus,
//!    Luuk van der Duim,
//!    Tait Hoyem

use atspi::connection::set_session_accessibility;
use atspi::proxy::accessible::{AccessibleProxy, ObjectRefExt};
use atspi::proxy::proxy_ext::ProxyExt;
use atspi::{DocumentEvents, Event, State};
use std::error::Error;
use tokio_stream::StreamExt;
use zbus::Connection;

/// recursively print the children of an accessible object
/// along with some useful information like their name and
/// coordinates
fn recursive_print_children<'a>(
	proxy: &'a AccessibleProxy<'a>,
	conn: &'a Connection,
	indent: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
	Box::pin(async move {
		for child in proxy.get_children().await?.iter() {
			let child_proxy = child.as_accessible_proxy(conn).await?;
			let component = child_proxy.proxies().await?.component().await?;
			let extents = component.get_extents(atspi::CoordType::Screen).await?;
			let name = child_proxy.name().await?;
			let role = child_proxy.get_role_name().await?;
			if child_proxy.get_state().await?.contains(State::Focusable) {
				println!(
					"{}child, role:{}, name:{}, extents:{:?}",
					" ".repeat(indent),
					role,
					name,
					extents
				);
			}
			recursive_print_children(&child_proxy, conn, indent + 2).await?;
		}
		Ok(())
	})
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;
	atspi.register_event::<DocumentEvents>().await?;

	let mut events = atspi.event_stream();

	while let Some(event) = events.next().await {
		match event {
			Ok(Event::Document(DocumentEvents::LoadComplete(ev))) => {
				let conn_clone = conn.clone();
				tokio::spawn(async move {
					let a11y_proxy = ev.item.into_accessible_proxy(&conn_clone).await;
					match a11y_proxy {
						Ok(proxy) => {
							if let Err(err) = recursive_print_children(&proxy, &conn_clone, 0).await
							{
								eprintln!("Error: {err}");
							}
						}
						Err(err) => eprintln!("Error creating proxy: {err}"),
					}
				});
			}
			Ok(_) => {}
			Err(err) => println!("Error: {err}"),
		}
	}
	Ok(())
}
