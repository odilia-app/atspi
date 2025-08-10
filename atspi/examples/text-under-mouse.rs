//! This example prints out the currently focused frame. A
//! frame is generally semantically equivalent to an application
//! window.
//!
//! ```sh
//! cargo run --example text-under-mouse
//! ```
//! Authors:
//!    Colton Loftus

use std::{collections::HashMap, error::Error, time::Duration};

use async_std::task::{self, sleep};
use atspi::{
	events::mouse::ButtonEvent,
	AtspiError, Event,
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
) -> Result<ObjectRef, Box<dyn Error>> {
	for app in apps.iter() {
		let proxy = app.clone().into_accessible_proxy(conn).await?;
		let state = proxy.get_state().await?;
		assert!(!state.contains(State::Active), "The top level application should never have active state; only its associated frames should have this state");

		for frame in proxy.get_children().await? {
			if frame
				.clone()
				.into_accessible_proxy(conn)
				.await?
				.get_state()
				.await?
				.contains(State::Active)
			{
				return Ok(frame.clone());
			}
		}
	}
	Err("There must be one active frame at any given time".into())
}

async fn find_relevant_descendant(
	children: Vec<ObjectRef>,
	conn: &zbus::Connection,
	x: i32,
	y: i32,
) -> Result<ObjectRef, Box<dyn Error>> {
	for child in children {
		let states = child.as_accessible_proxy(conn).await?.get_state().await?;
		if !states.contains(State::Showing) || !states.contains(State::Visible) {
			continue;
		}

		if child
			.as_accessible_proxy(conn)
			.await?
			.proxies()
			.await?
			.component()
			.await?
			.contains(x, y, atspi::CoordType::Window)
			.await?
		{
			let name = child.as_accessible_proxy(conn).await?.name().await?;
			println!(
				"Found object with accessible name '{name}' and objectref name '{}'",
				child.name
			);
			return Ok(child);
		}
	}
	Err("There is no relevant descendant".into())
}

async fn get_descendant_at_point<'a>(
	app_root: ObjectRef,
	conn: &'a zbus::Connection,
	x: i32,
	y: i32,
) -> Result<AccessibleProxy<'a>, Box<dyn Error>> {
	let mut accessible_at_point: ObjectRef = app_root.clone();

	let mut level = 0;
	loop {
		println!("descending to tree level {level}");
		level += 1;
		let deeper_accessible = accessible_at_point
			.as_accessible_proxy(conn)
			.await?
			.proxies()
			.await?
			.component()
			.await?
			.get_accessible_at_point(x, y, atspi::CoordType::Window)
			.await?;

		let role = deeper_accessible.as_accessible_proxy(conn).await?.get_role().await?;

		println!("Got deeper accessible with role: {role}");

		let children = deeper_accessible
			.as_accessible_proxy(conn)
			.await?
			.get_children()
			.await?;

		if children.is_empty() {
			// if there is no children then we are at the bottom and thus we can return
			println!("reached accessible with no children at bottom of tree");
			return Ok(deeper_accessible.into_accessible_proxy(conn).await?);
		}

		let descendant = find_relevant_descendant(children, conn, x, y).await;

		if let Err(e) = descendant {
			println!("Error: {e}");
			continue;
		} else {
			accessible_at_point = descendant.unwrap();
		}

		let accessible = accessible_at_point.clone().into_accessible_proxy(conn).await?;

		if let Ok(text_proxy) = accessible.proxies().await?.text().await {
			let text = text_proxy.get_text(0, 20000).await?;
			if !text.is_empty() {
				println!("Found text: '{text}'");
				return Ok(accessible);
			} else {
				println!("Had text proxy but contained empty text");
			}
		}

		sleep(Duration::from_millis(100)).await;
	}
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;
	atspi.register_event::<MouseEvents>().await?;

	let (tx, rx) = async_std::channel::bounded::<Result<Event, AtspiError>>(1);
	let atspi_clone = atspi.clone();
	task::spawn(async move {
		let mut events = atspi_clone.event_stream();
		while let Some(ev) = events.next().await {
			tx.send(ev).await.unwrap();
		}
	});

	let root = atspi.root_accessible_on_registry().await?;

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

	loop {
		match rx.recv().await {
			Ok(ev) => {
				if let Ok(ev) = <ButtonEvent>::try_from(ev.unwrap()) {
					let apps = atspi.root_accessible_on_registry().await?.get_children().await?;

					let active_frame = get_active_frame(apps, conn).await?;

					println!("\n\nActive frame: {}", active_frame.name);

					let (width, height) = active_frame
						.as_accessible_proxy(conn)
						.await?
						.proxies()
						.await?
						.component()
						.await?
						.get_position(atspi::CoordType::Screen)
						.await?;

					let x_relative_to_frame = ev.mouse_x - width;
					let y_relative_to_frame = ev.mouse_y - height;

					let app_name = id_to_name
						.get(
							&active_frame
								.as_accessible_proxy(conn)
								.await?
								.get_application()
								.await?
								.name
								.to_string(),
						)
						.unwrap();

					println!(
						"Converted absolute coords: {},{} to window relative coords: {},{}",
						ev.mouse_x, ev.mouse_y, x_relative_to_frame, y_relative_to_frame
					);

					println!(
						"Clicked on app '{app_name}' at {},{}",
						x_relative_to_frame, y_relative_to_frame
					);

					let component = get_descendant_at_point(
						active_frame,
						conn,
						x_relative_to_frame,
						y_relative_to_frame,
					)
					.await?;

					let text_offsets = component
						.proxies()
						.await?
						.text()
						.await?
						.get_offset_at_point(
							x_relative_to_frame,
							y_relative_to_frame,
							atspi::CoordType::Window,
						)
						.await?;
					println!("Clicked text offset: {text_offsets:?}");
					let text = component
						.proxies()
						.await?
						.text()
						.await?
						.get_text(0, text_offsets)
						.await?;

					println!("Clicked text: '{text}'");
				}
			}
			Err(err) => eprintln!("Error from event channel: {err}"),
		}
	}
}
