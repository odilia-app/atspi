mod common;

use atspi::events::EventInterfaces;
use atspi::identify::object::ObjectEvents;
use atspi::signify::Signified;
use atspi::Event;
use futures_lite::future::{block_on, race};
use futures_lite::pin;
use futures_lite::StreamExt;
use zbus::{fdo::DBusProxy, MatchRule, MessageType};

use crate::common::{a11y_bus_address, create_command, timeout};

#[test]
fn recv_active_signal() {
	let receive_good_event = async {
		let connection = atspi::AccessibilityBus::open().await.unwrap();
		let object_match_rule = MatchRule::builder()
			.msg_type(MessageType::Signal)
			.interface("org.a11y.atspi.Event.Object")
			.unwrap()
			.build();

		// This creates a DBus proxy object using the same connection as the AT-SPI proxy.
		let dbus_connection = DBusProxy::new(connection.connection()).await.unwrap();
		dbus_connection.add_match_rule(object_match_rule).await.unwrap();
		let a11y_event_stream = connection.event_stream();

		pin!(a11y_event_stream);

		let address = &*a11y_bus_address();
		let (method, kind, detail1, detail2, valuekind, value, props) =
			("StateChange", "active", "0", "0", "i", "0", "0");
		let command_output =
			create_command(address, method, kind, detail1, detail2, valuekind, value, props);

		assert_eq!(
			command_output.status.code().unwrap(),
			0,
			"Second `busctl` command existed with an failed status code."
		);

		while let Some(Ok(event)) = a11y_event_stream.next().await {
			let  Event::Interfaces(EventInterfaces::Object(ObjectEvents::StateChanged(change))) = event else { continue };
			assert!(change.kind() == "active");
			break;
		}
		Ok(())
	};

	let dur = std::time::Duration::from_secs(10);

	block_on(async {
		assert!(race(receive_good_event, timeout(dur)).await.is_ok());
	});
}
