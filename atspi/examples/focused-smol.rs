use atspi::{events::object::StateChangedEvent, ObjectEvents};
use futures_lite::stream::StreamExt;
use std::{error::Error, pin::pin};

smol_macros::main! {
	async fn main() -> Result<(), Box<dyn Error>> {
		let atspi = atspi::AccessibilityConnection::new().await?;
		atspi.register_event::<ObjectEvents>().await?;

		let mut events = atspi.event_stream();
		pin!(&mut events);

		while let Some(Ok(ev)) = events.next().await {
			let Ok(change) = <StateChangedEvent>::try_from(ev) else { continue };

			if change.state == "focused".into() && change.enabled {
				let bus_name = change.item.name_as_str().unwrap_or("unknown");
				println!("Accessible belonging to {bus_name}  focused!");
			}
		}
		Ok(())
	}
}
