use async_std::prelude::*;
use atspi::{
	events::{GenericEvent, HasMatchRules},
	identify::object::{ObjectEvents, StateChangedEvent},
	signify::Signified,
};
use enumflags2::BitFlag;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::Connection::open().await?;
	atspi.register_events(ObjectEvents::match_rules()?).await?;

	let events = atspi.event_stream();
	tokio::pin!(events);

	while let Some(Ok(ev)) = events.next().await {
		let Ok(change)  = <StateChangedEvent>::try_from(ev) else { continue };

		if change.kind() == "focused" && change.enabled() == 1 {
			let Some(bus_name) = change.inner().sender()? else { continue };
			println!("Accessible belonging to {bus_name}  focused!");
		}
	}
	Ok(())
}
