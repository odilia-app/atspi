use async_std::prelude::*;
use atspi::{
    events::GenericEvent,
    identify::object::StateChangedEvent,
    signify::Signified,
    zbus::{fdo::DBusProxy, MatchRule, MessageType},
};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let atspi = atspi::Connection::open().await?;
    atspi.register_event("Object").await?;

    let rule = MatchRule::builder()
        .msg_type(MessageType::Signal)
        .interface("org.a11y.atspi.Event.Object")?
        .build();

    let dbus = DBusProxy::new(atspi.connection()).await?;
    dbus.add_match_rule(rule).await?;

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
