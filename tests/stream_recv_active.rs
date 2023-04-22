mod common;

use atspi::events::EventInterfaces;
use atspi::identify::object::ObjectEvents;
use atspi::signify::Signified;
use atspi::Event;
use futures_lite::future::{block_on, race};
use futures_lite::pin;
use futures_lite::StreamExt;

use crate::common::{a11y_bus_address, create_command, timeout};

#[test]
fn recv_active_signal() {
	let receive_good_event = async {
		let connection = atspi::AccessibilityConnection::open().await.unwrap();
		connection.register_event::<ObjectEvents>().await.unwrap();
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
