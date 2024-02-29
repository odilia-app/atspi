use atspi_common::events::{AddAccessibleEvent, Event, RemoveAccessibleEvent};
use atspi_common::events::{CacheEvents, CACHE_ADD_SIGNATURE};
use atspi_common::{object_ref::ACCESSIBLE_PAIR_SIGNATURE, CacheItem, ObjectRef};
use atspi_connection::AccessibilityConnection;
use std::time::Duration;
use tokio_stream::StreamExt;
use zbus::Message;
use zvariant::OwnedObjectPath;

#[tokio::test]
async fn test_recv_remove_accessible() {
	let atspi = atspi_connection::AccessibilityConnection::new().await.unwrap();
	atspi.register_event::<RemoveAccessibleEvent>().await.unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg = {
		let unique_bus_name = atspi.connection().unique_name().unwrap();

		let remove_body = ObjectRef {
			name: ":69.420".into(),
			path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/remove").unwrap(),
		};

		let path = "/org/a11y/atspi/accessible/null";
		let iface = "org.a11y.atspi.Cache";
		let member = "RemoveAccessible";

		zbus::Message::signal(path, iface, member)
			.expect("Could not create signal")
			.sender(unique_bus_name)
			.expect("Could not set sender")
			.build(&remove_body)
			.unwrap()
	};

	assert_eq!(&msg.body().signature().unwrap(), &ACCESSIBLE_PAIR_SIGNATURE);
	atspi.connection().send(&msg).await.unwrap();

	loop {
		let to = events.try_next().await;
		let res = to.expect("Stream timed out").expect("Stream closed");
		let event = res.expect("Error: conversion to Event failed");

		if let Event::Cache(CacheEvents::Remove(event)) = event {
			let removed_accessible = event.node_removed;
			assert_eq!(removed_accessible.name.as_str(), ":69.420");
			assert_eq!(removed_accessible.path.as_str(), "/org/a11y/atspi/accessible/remove");
		} else {
			continue;
		}
	}
}

#[tokio::test]
async fn test_recv_add_accessible() {
	let atspi = AccessibilityConnection::new().await.unwrap();
	atspi.register_event::<AddAccessibleEvent>().await.unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg: zbus::Message = {
		let path = "/org/a11y/atspi/accessible/null";
		let iface = "org.a11y.atspi.Cache";
		let member = "AddAccessible";

		let unique_bus_name = atspi.connection().unique_name().unwrap();
		let add_body = CacheItem::default();

		zbus::Message::signal(path, iface, member)
			.expect("Could not create signal")
			.sender(unique_bus_name.clone())
			.expect("Could not set sender to {unique_bus_name:?}")
			.build(&add_body)
			.unwrap()
	};

	assert_eq!(&msg.body().signature().unwrap(), &CACHE_ADD_SIGNATURE);
	atspi
		.connection()
		.send(&msg)
		.await
		.expect("Message sending unsuccessful");

	loop {
		let to = events.try_next().await;
		let opt = to.expect("Stream timed out");
		let res = opt.expect("Stream closed");
		let event = res.expect("Error: conversion to Event failed");

		if let Event::Cache(CacheEvents::Add(AddAccessibleEvent { node_added, .. })) = event {
			assert_eq!(node_added.object.path.as_str(), "/org/a11y/atspi/accessible/object");
			assert_eq!(node_added.app.path.as_str(), "/org/a11y/atspi/accessible/application");
			assert_eq!(node_added.parent.path.as_str(), "/org/a11y/atspi/accessible/parent");
		} else {
			continue;
		}
	}
}

// It appears to be common practice to send the `Cache` signals with the
// body sent unmarshalled - with outer paretheses. This is a test to ensure
// that we can handle that case.
#[tokio::test]
async fn test_recv_add_accessible_unmarshalled_body() {
	let atspi = AccessibilityConnection::new().await.unwrap();
	atspi.register_event::<AddAccessibleEvent>().await.unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg: zbus::Message = {
		let path = "/org/a11y/atspi/accessible/null";
		let iface = "org.a11y.atspi.Cache";
		let member = "AddAccessible";

		let unique_bus_name = atspi.connection().unique_name().unwrap();

		let add_body = CacheItem::default();

		Message::signal(path, iface, member)
			.expect("Could not create signal")
			.sender(unique_bus_name.clone())
			.expect("Could not set sender to {unique_bus_name:?}")
			.build(&(add_body,)) // Note the (unnecessary) outer parens
			.unwrap()
	};

	assert_eq!(
		&msg.body()
			.signature()
			.expect("Message body does not have `AddAccessible` signature"),
		&CACHE_ADD_SIGNATURE
	);

	atspi
		.connection()
		.send(&msg)
		.await
		.expect("Message sending unsuccessful");

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
