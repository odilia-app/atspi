#[cfg(feature = "connection")]
use atspi::events::object::{ObjectEvents, StateChangedEvent};
#[cfg(feature = "connection")]
use futures_lite::stream::StreamExt;
#[cfg(feature = "connection")]
use std::error::Error;

#[cfg(feature = "connection")]
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::open().await?;
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
#[cfg(not(feature = "connection"))]
fn main() {
  println!("This test can not be run without the \"connection\" feature.");
}
