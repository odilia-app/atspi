//! This example prints out the currently focused frame. A
//! frame is generally semantically equivalent to an application
//! window.
//!
//! ```sh
//! cargo run --example text-under-mouse
//! ```
//! Authors:
//!    Colton Loftus

use std::{error::Error, time::Duration};

use async_std::task::sleep;
use atspi::{
	events::mouse::ButtonEvent,
	MouseEvents::{self},
	ObjectRef, State,
};
use atspi_connection::set_session_accessibility;
use atspi_proxies::{
	accessible::{AccessibleProxy, ObjectRefExt},
	proxy_ext::ProxyExt,
};
use futures_lite::stream::StreamExt;

async fn get_active_frame(
	apps: Vec<ObjectRef>,
	conn: &zbus::Connection,
) -> Result<AccessibleProxy, Box<dyn Error>> {
	for app in apps.iter() {
		let proxy = app.clone().into_accessible_proxy(conn).await?;
		let state = proxy.get_state().await?;
		assert!(!state.contains(State::Active), "The top level application should never have active state; only its associated frames should have this state");

		for frame in proxy.get_children().await? {
			let frame = frame.clone().into_accessible_proxy(conn).await?;
			let state = frame.get_state().await?;
			if state.contains(State::Active) {
				return Ok(frame);
			}
		}
	}
	Err("There must be one active frame at any given time".into())
}

async fn get_descendant_at_point<'a>(
	proxy: AccessibleProxy<'a>,
	conn: &'a zbus::Connection,
	x: i32,
	y: i32,
) -> Result<AccessibleProxy<'a>, Box<dyn Error>> {
	let mut component = proxy
		.proxies()
		.await?
		.component()
		.await?
		.get_accessible_at_point(x, y, atspi::CoordType::Screen)
		.await?
		.into_accessible_proxy(conn)
		.await?;

	loop {
		println!("descending");
		let deeper_component = component
			.proxies()
			.await?
			.component()
			.await?
			.get_accessible_at_point(x, y, atspi::CoordType::Screen)
			.await;

		let text_proxy = component.proxies().await?.text().await;

		if let Ok(text_proxy) = text_proxy {
			let text = text_proxy.get_text(0, i32::MAX).await?;
			if !text.is_empty() {
				println!("Found text: '{text}'");
				break;
			} else {
				println!("Had text proxy but contained empty text");
			}
		}

		match deeper_component {
			Ok(deeper_component) => {
				component = deeper_component.into_accessible_proxy(conn).await?;
			}
			Err(e) => {
				println!("error: {e}");
				break;
			}
		}
		sleep(Duration::from_millis(100)).await;
	}
	println!("descended to final component");
	Ok(component)
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;
	atspi.register_event::<MouseEvents>().await?;

	atspi.register_event::<MouseEvents>().await?;

	let mut events = atspi.event_stream();

	while let Some(ev) = events.next().await {
		match ev {
			Ok(ev) => {
				if let Ok(ev) = <ButtonEvent>::try_from(ev) {
					let apps = atspi.root_accessible_on_registry().await?.get_children().await?;

					let active_frame = get_active_frame(apps, conn).await?;

					let component =
						get_descendant_at_point(active_frame, conn, ev.mouse_x, ev.mouse_y).await?;

					let text =
						component.proxies().await?.text().await?.get_text(0, i32::MAX).await?;

					println!("Clicked text: '{text}'");
				}
			}
			Err(err) => eprintln!("Error: {err}"),
		}
	}

	Ok(())
}
