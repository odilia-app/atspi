use atspi_common::events::{signatures_are_eq, AddAccessibleEvent, Event, RemoveAccessibleEvent};
use atspi_common::events::{CacheEvents, CACHE_ADD_SIGNATURE};
use atspi_common::{
	assert_eq_signatures, object_reference::ACCESSIBLE_PAIR_SIGNATURE, CacheItem, InterfaceSet,
	ObjectRef, Role, StateSet,
};
use atspi_connection::AccessibilityConnection;
use std::time::Duration;
use tokio_stream::StreamExt;
use zbus::MessageBuilder;
use zvariant::OwnedObjectPath;

#[tokio::test]
async fn test_recv_remove_accessible() {
	let atspi = atspi_connection::AccessibilityConnection::open().await.unwrap();

	atspi.register_event::<RemoveAccessibleEvent>().await.unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg: zbus::Message = {
		let path = "/org/a11y/atspi/accessible/null";
		let iface = "org.a11y.atspi.Cache";
		let member = "RemoveAccessible";

		let unique_bus_name = atspi.connection().unique_name().unwrap();
		let remove_body = ObjectRef {
			name: ":69.420".into(),
			path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/remove").unwrap(),
		};

		MessageBuilder::signal(path, iface, member)
			.expect("Could not create signal")
			.sender(unique_bus_name.clone())
			.expect("Could not set sender to {unique_bus_name:?}")
			.build(&remove_body)
			.unwrap()
	};

	assert_eq_signatures!(&msg.body_signature().unwrap(), &ACCESSIBLE_PAIR_SIGNATURE);
	atspi.connection().send_message(msg).await.unwrap();

	loop {
		let to = events.try_next().await;
		assert!(to.is_ok(), "Stream timed out");
		let opt = to.unwrap();

		match opt {
			Some(res) => {
				match res {
					Ok(event) => match event {
						Event::Cache(CacheEvents::Remove(event)) => {
							let removed_accessible = event.node_removed;
							assert_eq!(
								removed_accessible.path.as_str(),
								"/org/a11y/atspi/accessible/remove"
							);
							break;
						}
						_ => continue,
					},
					// Stream yields a Some(Err(Error)) when a message is received
					Err(e) => panic!("Error: conversion to Event failed {e:?}"),
				}
			}
			// Stream yields a None when the stream is closed
			None => panic!("Stream closed"),
		}
	}
}

#[tokio::test]
async fn test_recv_add_accessible() {
	let atspi = AccessibilityConnection::open().await.unwrap();
	atspi.register_event::<AddAccessibleEvent>().await.unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg: zbus::Message = {
		let path = "/org/a11y/atspi/accessible/null";
		let iface = "org.a11y.atspi.Cache";
		let member = "AddAccessible";

		let unique_bus_name = atspi.connection().unique_name().unwrap();

		let add_body = CacheItem {
			object: ObjectRef {
				name: ":1.1".to_string(),
				path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/object").unwrap(),
			},
			app: ObjectRef {
				name: ":1.1".to_string(),
				path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/application").unwrap(),
			},
			parent: ObjectRef {
				name: ":1.1".to_string(),
				path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/parent").unwrap(),
			},
			index: 0,
			children: 0,
			ifaces: InterfaceSet::empty(),
			short_name: String::new(),
			role: Role::Application,
			name: "Hi".to_string(),
			states: StateSet::empty(),
		};

		MessageBuilder::signal(path, iface, member)
			.expect("Could not create signal")
			.sender(unique_bus_name.clone())
			.expect("Could not set sender to {unique_bus_name:?}")
			.build(&add_body)
			.unwrap()
	};

	assert_eq_signatures!(
		&msg.body_signature()
			.expect("marshalled AddObjectRef body signature != expected"),
		&CACHE_ADD_SIGNATURE
	);
	atspi
		.connection()
		.send_message(msg)
		.await
		.expect("Message sending unsuccesful");

	loop {
		let to = events.try_next().await;
		assert!(to.is_ok(), "Stream timed out");
		let opt = to.unwrap();

		match opt {
			Some(res) => {
				// This result comes from inner event-stream, Stream yields a Result<Event, AtspiError>
				match res {
					Ok(event) => match event {
						Event::Cache(CacheEvents::Add(AddAccessibleEvent {
							item: _,
							node_added: cache_item,
						})) => {
							assert_eq!(
								cache_item.object.path.as_str(),
								"/org/a11y/atspi/accessible/object"
							);
							assert_eq!(
								cache_item.app.path.as_str(),
								"/org/a11y/atspi/accessible/application"
							);
							assert_eq!(
								cache_item.parent.path.as_str(),
								"/org/a11y/atspi/accessible/parent"
							);
							break;
						}
						_any_other_event => continue,
					},
					Err(e) => panic!("Error: conversion to Event failed {e:?}"),
				}
			}
			// Stream yields a None when the stream is closed
			None => panic!("Stream closed"),
		}
	}
}
