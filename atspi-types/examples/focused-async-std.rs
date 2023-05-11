use async_std::prelude::*;
use atspi::events::object::{ObjectEvents, StateChangedEvent};
use std::error::Error;
use futures_lite::stream::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi_client::AccessibilityConnection::open().await?;
	atspi.register_event::<ObjectEvents>().await?;

	let events = atspi.event_stream();
	tokio::pin!(events);

	while let Some(Ok(ev)) = events.next().await {
		let Ok(change)  = <StateChangedEvent>::try_from(ev) else { continue };

		if change.state == "focused" && change.enabled == 1 {
			let bus_name = change.item.name.clone();
			println!("Accessible belonging to {bus_name}  focused!");
		}
	}
	Ok(())
}