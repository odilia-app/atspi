use serial_test::*;

#[cfg(feature = "unstable-traits")]
use atspi::accessible_ext::AccessibleExt;
#[cfg(feature = "unstable-traits")]
use atspi::convertable::Convertable;
use atspi::{
	accessible::{AccessibleProxy, Role},
	collection::MatchType,
	events::{Event, GenericEvent},
	identify::object::{ChildrenChangedEvent, ObjectEvents, StateChangedEvent},
	set_session_accessibility, AccessibilityConnection, Interface, InterfaceSet, StateSet,
};
use fantoccini::{Client, ClientBuilder};
use futures_lite::stream::StreamExt;
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};
use tokio::time::{sleep, timeout, Duration};

async fn start_firefox(port: &str) -> Client {
	Command::new("geckodriver")
		.arg("-p")
		.arg(port)
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.stdin(Stdio::null())
		.spawn()
		.expect("Could not create instance of geckodriver");
	sleep(Duration::from_millis(50)).await;
	/*"args": ["-headless"],*/
	let options = serde_json::from_str(
		r#"
{
"moz:firefoxOptions": {
	"prefs": {
		"dom.ipc.processCount": 8,
		"javascript.options.showInConsole": false
	},
	"log": { "level": "trace" },
	"env": {
		"MOZ_LOG": "nsHttp:5",
		"MOZ_LOG_FILE": "ff.log"
	}
} 
}
"#,
	)
	.expect("Could not create value");
	ClientBuilder::native()
		.capabilities(options)
		.connect(&format!("http://localhost:{}", port))
		.await
		.expect("failed to connect to WebDriver")
}

#[cfg(feature = "unstable-traits")]
#[tokio::test]
#[serial]
async fn test_get_next_accessible_ext() -> Result<(), Box<dyn std::error::Error>> {
	set_session_accessibility(true).await?;
	let c = start_firefox("4445").await;
	let con = AccessibilityConnection::open().await?;

	// first, go to the Wikipedia page for Foobar
	c.goto("file:///home/tait/Documents/atspi/index.html").await?;
	let con = AccessibilityConnection::open().await?;
	con.register_event::<ChildrenChangedEvent>().await?;
	let mut events = con.event_stream();
	std::pin::pin!(&mut events);
	loop {
		let e = timeout(Duration::from_millis(1000), events.next()).await;
		match e {
			Ok(Some(Ok(Event::Object(ObjectEvents::ChildrenChanged(event))))) => {
				if !event.operation.contains("add") {
					continue;
				}
				let accessible = AccessibleProxy::builder(con.connection())
					.destination(event.child.name)?
					.path(event.child.path)?
					.build()
					.await?;
				let interfaces = accessible.get_interfaces().await?;
				let args = (
					vec![Role::Heading],
					MatchType::Empty,
					HashMap::new(),
					MatchType::Empty,
					InterfaceSet::empty(),
					MatchType::Empty,
				);
				let mut empty_visited = Vec::new();
				let get_next = timeout(
					Duration::from_millis(100),
					accessible.get_next(&args, true, &mut empty_visited),
				);
				match get_next.await {
					Err(_) => {
						println!("This took too long to run; this ususally means DBus hung on us.");
					}
					Ok(Ok(Some(a11y))) => {
						println!("We got an answer!");
					}
					Ok(Ok(None)) => {
						println!("We didn't get an answer!");
					}
					Ok(Err(e)) => {
						println!("{:?}", e);
					}
				};
			}
			Err(_) => {
				c.clone().close().await;
				return Ok(());
			}
			_ => {
				panic!("Unexpected event!");
			}
		}
	}
	Ok(())
}

#[cfg(feature = "unstable-traits")]
macro_rules! conversion_test {
	($ifaces:expr, $acc:expr, $iface:expr, $to_func:ident) => {
		if $ifaces.contains($iface) {
			let new = $acc.$to_func().await?;
			assert_eq!(new.path(), $acc.path());
		}
	};
}

#[cfg(feature = "unstable-traits")]
#[tokio::test]
#[serial]
async fn test_convertable_trait() -> Result<(), Box<dyn std::error::Error>> {
	set_session_accessibility(true).await?;
	let c = start_firefox("4445").await;

	// first, go to the Wikipedia page for Foobar
	c.goto("file:///home/tait/Documents/atspi/index.html").await?;
	let con = AccessibilityConnection::open().await?;
	con.register_event::<ChildrenChangedEvent>().await?;
	let mut events = con.event_stream();
	std::pin::pin!(&mut events);
	loop {
		let e = timeout(Duration::from_millis(1000), events.next()).await;
		match e {
			Ok(Some(Ok(Event::Object(ObjectEvents::ChildrenChanged(event))))) => {
				if !event.operation.contains("add") {
					continue;
				}
				let accessible = AccessibleProxy::builder(con.connection())
					.destination(event.child.name)?
					.path(event.child.path)?
					.build()
					.await?;
				let interfaces = accessible.get_interfaces().await?;
				conversion_test!(interfaces, accessible, Interface::Action, to_action);
				conversion_test!(interfaces, accessible, Interface::Application, to_application);
				conversion_test!(interfaces, accessible, Interface::Collection, to_collection);
				conversion_test!(interfaces, accessible, Interface::Component, to_component);
				conversion_test!(interfaces, accessible, Interface::Document, to_document);
				conversion_test!(interfaces, accessible, Interface::EditableText, to_editable_text);
				conversion_test!(interfaces, accessible, Interface::Hyperlink, to_hyperlink);
				conversion_test!(interfaces, accessible, Interface::Hypertext, to_hypertext);
				conversion_test!(interfaces, accessible, Interface::Image, to_image);
				conversion_test!(interfaces, accessible, Interface::Selection, to_selection);
				conversion_test!(interfaces, accessible, Interface::Table, to_table);
				conversion_test!(interfaces, accessible, Interface::TableCell, to_table_cell);
				conversion_test!(interfaces, accessible, Interface::Text, to_text);
				conversion_test!(interfaces, accessible, Interface::Value, to_value);
			}
			Err(e) => {
				// elapsed is allowed, this means the test ran fine; the test will panic if something goes wrong
				c.close().await;
				return Ok(());
			}
			_ => {
				panic!("An unexpected value was received from the future.");
			}
		}
	}
	Ok(())
}

macro_rules! assert_no_err {
	($future:expr, $client:expr) => {
		if $future.await.is_err() {
			$client.clone().close().await;
			assert!(false);
		}
	};
}

#[tokio::test]
#[serial]
async fn test_interface_accessible_methods() -> Result<(), Box<dyn std::error::Error>> {
	set_session_accessibility(true).await?;
	let c = start_firefox("4445").await;

	// first, go to the Wikipedia page for Foobar
	c.goto("file:///home/tait/Documents/atspi/index.html").await?;
	let con = AccessibilityConnection::open().await?;
	con.register_event::<ChildrenChangedEvent>().await?;
	let mut events = con.event_stream();
	std::pin::pin!(&mut events);
	loop {
		let e = timeout(Duration::from_millis(1000), events.next()).await;
		match e {
			Ok(Some(Ok(Event::Object(ObjectEvents::ChildrenChanged(event))))) => {
				if !event.operation.contains("add") {
					continue;
				}
				let accessible = AccessibleProxy::builder(con.connection())
					.destination(event.child.name)?
					.path(event.child.path)?
					.build()
					.await
					.expect("Unable to create AccessibleProxy");
				let interfaces = accessible
					.get_interfaces()
					.await
					.expect("Unable to build an interface set from get_interfaces response");
				if interfaces.contains(Interface::Accessible) {
					assert_no_err!(accessible.get_application(), c);
					assert_no_err!(accessible.get_attributes(), c);
					assert_no_err!(accessible.get_child_at_index(0), c);
					assert_no_err!(accessible.get_index_in_parent(), c);
					assert_no_err!(accessible.get_localized_role_name(), c);
					assert_no_err!(accessible.get_relation_set(), c);
					assert_no_err!(accessible.get_role(), c);
					assert_no_err!(accessible.get_role_name(), c);
					assert_no_err!(accessible.get_state(), c);
					assert_no_err!(accessible.accessible_id(), c);
					assert_no_err!(accessible.child_count(), c);
					assert_no_err!(accessible.description(), c);
					assert_no_err!(accessible.locale(), c);
					assert_no_err!(accessible.name(), c);
					assert_no_err!(accessible.parent(), c);
				}
			}
			Err(e) => {
				// elapsed is allowed, this means the test ran fine; the test will panic if something goes wrong
				c.close().await;
				return Ok(());
			}
			_ => {
				panic!("An unexpected value was received from the future.");
			}
		}
	}
	Ok(())
}

#[cfg(feature = "unstable-traits")]
#[tokio::test]
#[serial]
async fn test_text_methods() -> Result<(), Box<dyn std::error::Error>> {
	set_session_accessibility(true).await?;
	let c = start_firefox("4445").await;

	// first, go to the Wikipedia page for Foobar
	c.goto("file:///home/tait/Documents/atspi/index.html").await?;
	let con = AccessibilityConnection::open().await?;
	con.register_event::<ChildrenChangedEvent>().await?;
	let mut events = con.event_stream();
	std::pin::pin!(&mut events);
	loop {
		let e = timeout(Duration::from_millis(1000), events.next()).await;
		match e {
			Ok(Some(Ok(Event::Object(ObjectEvents::ChildrenChanged(event))))) => {
				if !event.operation.contains("add") {
					continue;
				}
				let accessible = AccessibleProxy::builder(con.connection())
					.destination(event.child.name)?
					.path(event.child.path)?
					.build()
					.await
					.expect("Unable to create AccessibleProxy");
				let interfaces = accessible
					.get_interfaces()
					.await
					.expect("Unable to build an interface set from get_interfaces response");
				if interfaces.contains(Interface::Text) {
					let Ok(text) = accessible.to_text().await else {
						continue;
					};
					assert_no_err!(text.character_count(), c);
					assert_no_err!(text.caret_offset(), c);
					assert_no_err!(text.set_caret_offset(0), c);
					assert_no_err!(text.get_default_attributes(), c);
					assert_no_err!(text.get_default_attribute_set(), c);
				}
			}
			Err(e) => {
				// elapsed is allowed, this means the test ran fine; the test will panic if something goes wrong
				c.close().await;
				return Ok(());
			}
			_ => {
				panic!("An unexpected value was received from the future.");
			}
		}
	}
	Ok(())
}
