//! This example prints out the text under the mouse click.
//! This example only works with X11 given the fact
//! Wayland does not support global coordinates or
//! global input events.
//!
//! ```sh
//! cargo run --example text-under-click
//! ```
//! Authors:
//!    Colton Loftus

use atspi::{
	events::mouse::ButtonEvent,
	AtspiError, Event,
	MouseEvents::{self},
	ObjectRefOwned, State,
};
use atspi_connection::set_session_accessibility;
use atspi_proxies::{
	accessible::{AccessibleProxy, ObjectRefExt},
	proxy_ext::ProxyExt,
};
use futures_lite::stream::StreamExt;
use std::{collections::HashMap, error::Error};
use tokio::task;

// macro to convert an ObjectRef into an AccessibleProxy
// this macro is used to avoid repeated code for converting
// an ObjectRef into an AccessibleProxy and handling errors
// it can be used in two ways: `into` or `as`
// `into` will convert the ObjectRef into an AccessibleProxy
// `as` will convert the ObjectRef into an AccessibleProxy if it is not already one
// It can either return an error or continue the loop
//
// the first argument is either `into` or `as`
// the second argument is either `continue` or `error`
// the third argument is the connection to use for the conversion
// the fourth argument is the ObjectRef to convert
// the fifth argument is the context string to print in case of an error
macro_rules! accessible_proxy {
	(into, continue, $conn:expr, $object_ref:expr, $ctx:expr) => {{
		match $object_ref.clone().into_accessible_proxy($conn).await {
			Ok(proxy) => proxy,
			Err(AtspiError::NullRef(msg)) => {
				eprintln!("L{}: {}: {}", line!(), $ctx, msg);
				continue;
			}
			Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error>),
		}
	}};
	(into, error, $conn:expr, $object_ref:expr, $ctx:expr) => {{
		match $object_ref.clone().into_accessible_proxy($conn).await {
			Ok(proxy) => proxy,
			Err(AtspiError::NullRef(msg)) => {
				eprintln!("L{}: {}: {}", line!(), $ctx, msg);
				return Err(Box::new(AtspiError::NullRef(msg)));
			}
			Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error>),
		}
	}};
	(as, continue, $conn:expr, $object_ref:expr, $ctx:expr) => {{
		match $object_ref.as_accessible_proxy($conn).await {
			Ok(proxy) => proxy,
			Err(AtspiError::NullRef(msg)) => {
				eprintln!("L{}: {}: {}", line!(), $ctx, msg);
				continue;
			}
			Err(err) => return Err(Box::new(err)),
		}
	}};
	(as, error, $conn:expr, $object_ref:expr, $ctx:expr) => {{
		match $object_ref.as_accessible_proxy($conn).await {
			Ok(proxy) => proxy,
			Err(AtspiError::NullRef(msg)) => {
				eprintln!("L{}: {}: {}", line!(), $ctx, msg);
				return Err(Box::new(AtspiError::NullRef(msg)));
			}
			Err(err) => return Err(Box::new(err)),
		}
	}};
}

async fn get_active_frame(
	apps: Vec<ObjectRefOwned>,
	conn: &zbus::Connection,
) -> Result<ObjectRefOwned, Box<dyn Error>> {
	for app in apps.iter() {
		let proxy = accessible_proxy!(into, continue, conn, app, "`get_active_frame`");
		let state = proxy.get_state().await?;

		assert!(!state.contains(State::Active), "The top level application should never have active state; only its associated frames should have this state");

		for frame in proxy.get_children().await? {
			let candidate = accessible_proxy!(into, continue, conn, frame, "`get_active_frame`");

			if candidate.get_state().await?.contains(State::Active) {
				return Ok(frame);
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
	children: Vec<ObjectRefOwned>,
	conn: &zbus::Connection,
	x: i32,
	y: i32,
) -> Result<ObjectRefOwned, Box<dyn Error>> {
	for child in &children {
		let child_proxy =
			accessible_proxy!(as, continue, conn, child, "`find_relevant_descendant`");

		// Unclear if this is even necessary since it seems like get_accessible_at_point
		// tends to return the proxy accessible anyways
		// Orca checks for state while descending so will keep it for now
		let states = child_proxy.get_state().await?;
		if !states.contains(State::Showing) || !states.contains(State::Visible) {
			continue;
		}

		if child_proxy
			.proxies()
			.await?
			.component()
			.await?
			.contains(x, y, atspi::CoordType::Window)
			.await?
		{
			let name = child_proxy.name().await?;
			println!(
				"Found object with accessible name '{name}' and bus name '{}'",
				child
					.name_as_str()
					.expect("we managed a bus call, so this should be valid")
			);
			return Ok(child.clone());
		}
	}

	Err(Box::new(NoRelevantDescendantError { children: children.len() }))
}

async fn get_descendant_at_point<'a>(
	frame_root: ObjectRefOwned,
	conn: &'a zbus::Connection,
	x: i32,
	y: i32,
) -> Result<AccessibleProxy<'a>, Box<dyn Error>> {
	let mut accessible_at_point: ObjectRefOwned = frame_root.clone();

	let mut level = 0;
	loop {
		println!("Descended {level} levels");
		level += 1;
		let deeper_accessible =
			accessible_proxy!(as, continue, conn, accessible_at_point, "`get_descendant_at_point`")
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

		let role =
			accessible_proxy!(as, continue, conn, deeper_accessible, "`get_descendant_at_point`")
				.get_role()
				.await?;

		println!("Found accessible with role: {role}");

		let children =
			accessible_proxy!(as, continue, conn, deeper_accessible, "`get_descendant_at_point`")
				.get_children()
				.await?;

		if children.is_empty() {
			println!("Reached accessible with no children at bottom of tree");
			return Ok(accessible_proxy!(
				into,
				error,
				conn,
				deeper_accessible,
				"`get_descendant_at_point`"
			));
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
					return Ok(accessible_proxy!(
						into,
						error,
						conn,
						accessible_at_point,
						"`get_descendant_at_point`"
					));
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
		let child_proxy = accessible_proxy!(into, continue, conn, child.clone(), "`main loop`");

		let natural_name = child_proxy.name().await.unwrap();
		let id = child_proxy
			.get_application()
			.await
			.unwrap()
			.name()
			.unwrap()
			.to_string();
		id_to_name.insert(id, natural_name);
	}

	loop {
		if let Some(ev) = rx.recv().await {
			if let Ok(ev) = <ButtonEvent>::try_from(ev.unwrap()) {
				let apps = root.get_children().await.unwrap();
				let active_frame = get_active_frame(apps, conn).await.unwrap();

				let active_frame_proxy =
					accessible_proxy!(into, error, conn, active_frame, "`main loop`");

				let (width, height) = active_frame_proxy
					.proxies()
					.await?
					.component()
					.await?
					.get_position(atspi::CoordType::Screen)
					.await?;

				let x_relative_to_frame = ev.mouse_x - width;
				let y_relative_to_frame = ev.mouse_y - height;

				let id_key = active_frame_proxy
					.get_application()
					.await?
					.name_as_str()
					.ok_or("null")?
					.to_string();

				let unknown = String::from("unknown");
				let app_name = id_to_name.get(&id_key).unwrap_or(&unknown);
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
