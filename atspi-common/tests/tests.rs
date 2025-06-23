use atspi_common::events::cache::{AddAccessibleEvent, RemoveAccessibleEvent};
use atspi_common::events::CacheEvents;
use atspi_common::events::Event;
use atspi_common::{CacheItem, ObjectRef};
use atspi_connection::AccessibilityConnection;
use std::time::Duration;
use tokio_stream::StreamExt;
use zbus::Message;
use zbus_names::OwnedUniqueName;
use zvariant::OwnedObjectPath;

#[tokio::test]
async fn test_recv_remove_accessible() {
	let atspi = atspi_connection::AccessibilityConnection::new().await.unwrap();
	atspi.register_event::<RemoveAccessibleEvent>().await.unwrap();
	let unique_bus_name = atspi.connection().unique_name().unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg = {
		let remove_body = ObjectRef {
			name: OwnedUniqueName::try_from(":69.420").unwrap(),
			path: OwnedObjectPath::try_from("/org/a11y/atspi/accessible/remove").unwrap(),
		};

		let path = "/org/a11y/atspi/accessible/cache";
		let iface = "org.a11y.atspi.Cache";
		let member = "RemoveAccessible";

		zbus::Message::signal(path, iface, member)
			.expect("Could not create signal")
			.sender(&unique_bus_name.clone())
			.expect("Could not set sender")
			.build(&remove_body)
			.unwrap()
	};

	// Retrieve the signature of the body of 'RemoveAccessible' signal from XML
	// and compare it to the signature of the body of the message
	assert_eq!(
		msg.body().signature(),
		&zbus_lockstep::signal_body_type_signature!("RemoveAccessible")
	);

	atspi.connection().send(&msg).await.unwrap();

	loop {
		let to = events.try_next().await;
		let event = to
			.expect("stream timed out")
			.expect("stream closed")
			.expect("conversion to `Event` failed");

		if let Event::Cache(CacheEvents::Remove(event)) = event {
			// If we were not sender of the signal, continue listening.
			if event.item.name.as_str() != unique_bus_name.as_str() {
				continue;
			}

			let ObjectRef { name, path } = event.node_removed;
			assert_eq!(name.as_str(), ":69.420");
			assert_eq!(path.as_str(), "/org/a11y/atspi/accessible/remove");

			// If we were sender, break the loop.
			break;
		}
	}
}

#[tokio::test]
async fn test_recv_add_accessible() {
	let atspi = AccessibilityConnection::new().await.unwrap();
	atspi.register_event::<AddAccessibleEvent>().await.unwrap();
	let unique_bus_name = atspi.connection().unique_name().unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg: zbus::Message = {
		let path = "/org/a11y/atspi/cache";
		let iface = "org.a11y.atspi.Cache";
		let member = "AddAccessible";

		let add_body = CacheItem::default();

		zbus::Message::signal(path, iface, member)
			.expect("could not create signal")
			.sender(&unique_bus_name.clone())
			.expect("could not set sender")
			.build(&add_body)
			.unwrap()
	};

	assert_eq!(
		msg.body().signature(),
		&zbus_lockstep::signal_body_type_signature!("AddAccessible")
	);

	atspi
		.connection()
		.send(&msg)
		.await
		.expect("Message sending unsuccessful");

	loop {
		let to = events.try_next().await;
		let event = to
			.expect("stream timed out")
			.expect("stream closed")
			.expect("conversion to `Event` failed");

		if let Event::Cache(CacheEvents::Add(AddAccessibleEvent { node_added, item })) = event {
			// If we did not send the signal, continue listening.
			if item.name.as_str() != unique_bus_name.as_str() {
				continue;
			}
			assert_eq!(node_added.object.path.as_str(), "/org/a11y/atspi/accessible/object");
			assert_eq!(node_added.app.path.as_str(), "/org/a11y/atspi/accessible/application");

			let parent: ObjectRef = node_added.parent.into();
			assert_eq!(parent.path.as_str(), "/org/a11y/atspi/accessible/parent");

			// If we did, break the loop.
			break;
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
	let unique_bus_name = atspi.connection().unique_name().unwrap();

	let events = tokio_stream::StreamExt::timeout(atspi.event_stream(), Duration::from_secs(1));
	tokio::pin!(events);

	let msg: zbus::Message = {
		let path = "/org/a11y/atspi/accessible/null";
		let iface = "org.a11y.atspi.Cache";
		let member = "AddAccessible";

		let add_body = CacheItem::default();

		Message::signal(path, iface, member)
			.expect("Could not create Builder for signal")
			.sender(&unique_bus_name.clone())
			.expect("Could not set sender")
			.build(&(add_body)) // <--- Note the unmarshalled body
			.unwrap()
	};

	assert_eq!(
		msg.body().signature(),
		&zbus_lockstep::signal_body_type_signature!("AddAccessible")
	);

	atspi
		.connection()
		.send(&msg)
		.await
		.expect("Message sending unsuccessful");

	loop {
		let to = events.try_next().await;
		let event = to
			.expect("stream timed out")
			.expect("stream closed")
			.expect("conversion to `Event` failed");

		if let Event::Cache(CacheEvents::Add(AddAccessibleEvent { node_added, item })) = event {
			// If we did not send the signal, continue listening.
			if item.name.as_str() != unique_bus_name.as_str() {
				continue;
			}

			assert_eq!(node_added.object.path.as_str(), "/org/a11y/atspi/accessible/object");
			assert_eq!(node_added.app.path.as_str(), "/org/a11y/atspi/accessible/application");

			let parent: ObjectRef = node_added.parent.into();
			// Check that the parent is correctly converted from `ParentRef` to `ObjectRef

			assert_eq!(parent.path.as_str(), "/org/a11y/atspi/accessible/parent");

			// If we did, break the loop.
			break;
		}
	}
}
