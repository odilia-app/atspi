pub trait EventExtError: crate::event::Event {
	type Error: std::error::Error;
}

pub trait EventExt {
}

impl<T: EventExtError + crate::event::Event> EventExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	event_ext::EventExt,
	event::{EventProxy,
	EventProxyBlocking}};	fn implements_event_ext<T: EventExt>() {}
	#[test]
	fn check_event_implements_event_ext() {
		implements_event_ext::<EventProxy<'static>>();
	}
	#[test]
	fn check_blocking_event_implements_event_ext() {
		implements_event_ext::<EventProxyBlocking<'static>>();
	}
}