//! This example prints out the text under the mouse click.
//! This example only works with X11 given the fact
//! Wayland does not support global coordinates or
//! global input events.
//!
//! ```sh
//! cargo run --example text-under-mouse
//! ```
//! Authors:
//!    Colton Loftus

use std::{collections::HashMap, error::Error};

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
use tokio::task;

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

#[derive(Debug)]
struct NoRelevantDescendantError {
	children: usize,
}
impl std::error::Error for NoRelevantDescendantError {}
impl std::fmt::Display for NoRelevantDescendantError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "No relevant descendant found among {} children", self.children)
	}
}

async fn find_relevant_descendant(
	children: Vec<ObjectRef>,
	conn: &zbus::Connection,
	x: i32,
	y: i32,
) -> Result<ObjectRef, Box<dyn Error>> {
	for child in &children {
		// Unclear if this is even necessary since it seems like get_accessible_at_point
		// tends to return the proxy accessible anyways
		// Orca checks for state while descending so will keep it for now
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
			return Ok(child.clone());
		}
	}

	Err(Box::new(NoRelevantDescendantError { children: children.len() }))
}

async fn get_descendant_at_point<'a>(
	frame_root: ObjectRef,
	conn: &'a zbus::Connection,
	x: i32,
	y: i32,
) -> Result<AccessibleProxy<'a>, Box<dyn Error>> {
	let mut accessible_at_point: ObjectRef = frame_root.clone();

	let mut level = 0;
	loop {
		println!("Descended {level} levels");
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

		if deeper_accessible == accessible_at_point {
			// unclear if this can ever be the case; doesn't seem to be in testing
			println!("got the same accessible twice; thus indicated that we reached the bottom of the tree");
			return Ok(accessible_at_point.clone().into_accessible_proxy(conn).await?);
		}

		let role = deeper_accessible.as_accessible_proxy(conn).await?.get_role().await?;

		println!("Found accessible with role: {role}");

		let children = deeper_accessible
			.as_accessible_proxy(conn)
			.await?
			.get_children()
			.await?;

		if children.is_empty() {
			println!("Reached accessible with no children at bottom of tree");
			return Ok(deeper_accessible.into_accessible_proxy(conn).await?);
		}

		match find_relevant_descendant(children, conn, x, y).await {
			Ok(descendant) => {
				// If we found a relevant descendant, we use it as
				// the new accessible at the point and continue descending
				accessible_at_point = descendant;
			}
			Err(err) => match err.downcast::<NoRelevantDescendantError>() {
				Ok(err) => {
					// If there are no relevant children, we're at the bottom of the tree
					// and thus return the accessible at the point
					println!("{err}; returning early");
					return Ok(accessible_at_point.clone().into_accessible_proxy(conn).await?);
				}
				Err(err) => {
					// If we got a different error, something went wrong
					// and we should return the error
					eprintln!("Unexpected error: {err}");
					return Err(err);
				}
			},
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	let conn = atspi.connection();
	set_session_accessibility(true).await?;
	atspi.register_event::<MouseEvents>().await?;

	// Must use a separate task here to avoid hangs;
	// zbus's internal buffer can potentially fill up
	// if you wait too long between events.next.await calls
	let (tx, mut rx) = tokio::sync::mpsc::channel::<Result<Event, AtspiError>>(1);
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
		if let Some(ev) = rx.recv().await {
			if let Ok(ev) = <ButtonEvent>::try_from(ev.unwrap()) {
				let apps = root.get_children().await?;

				let active_frame = get_active_frame(apps, conn).await?;

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

				let unknown = String::from("unknown");
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
					.unwrap_or(&unknown);

				println!(
						"\n\nClicked on app '{app_name}' at absolute coords: {},{} and window relative coords: {},{}",
						ev.mouse_x, ev.mouse_y, x_relative_to_frame, y_relative_to_frame
					);

				let component_with_clicked_text = get_descendant_at_point(
					active_frame,
					conn,
					x_relative_to_frame,
					y_relative_to_frame,
				)
				.await?;

				let text_proxy = component_with_clicked_text.proxies().await?.text().await;

				if let Ok(text_proxy) = text_proxy {
					let text_offset_length = text_proxy
						.get_offset_at_point(
							x_relative_to_frame,
							y_relative_to_frame,
							atspi::CoordType::Window,
						)
						.await?;
					println!("Clicked accessible has text offset of size {text_offset_length:?}");
					let all_text = text_proxy.get_text(0, text_offset_length).await?;
					println!("User clicked on text: '{all_text}'");
				} else {
					eprintln!("Did not find text proxy; nothing to print");
				}
			}
		}
	}
}
