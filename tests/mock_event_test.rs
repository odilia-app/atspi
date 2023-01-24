mod common;

use atspi::events::EventInterfaces;
use atspi::identify::object::ObjectEvents;
use atspi::signify::Signified;
use atspi::Event;
use futures_lite::future::block_on;
use futures_lite::stream::once;
use futures_lite::{pin, StreamExt};

#[test]
fn recv_mockup() {
    assert!(block_on(mockup_signal()).is_ok());
}

async fn mockup_signal() -> Result<(), ()> {
    let msg = common::valid_mockup_message(
        "TextCaretMoved",
        "org.a11y.atspi.Event.Object",
        "/org/a11y/atspi/pbject/event/1234",
    );

    let a11y_stream = once(Event::try_from(msg));
    pin!(a11y_stream);

    while let Some(Ok(event)) = a11y_stream.next().await {
        let Event::Interfaces(EventInterfaces::Object(ObjectEvents::TextCaretMoved(ev))) = event else { continue };

        if ev.kind() == "test body" {
            break;
        }
    }
    Ok(())
}
