use atspi::events::object::StateChangedEvent;
use atspi::events::ObjectEvents;
use futures_lite::stream::StreamExt;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::new().await?;
	atspi.register_event::<ObjectEvents>().await?;

	let events = atspi.event_stream();
	tokio::pin!(events);

	while let Some(Ok(ev)) = events.next().await {
		let Ok(change) = <StateChangedEvent>::try_from(ev) else { continue };

		if change.state == "focused".into() && change.enabled {
			let bus_name = change.item.name.clone();
			println!("Accessible belonging to {bus_name}  focused!");
		}
	}
	Ok(())
}
