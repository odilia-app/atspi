use crate::event::{EventProxy, EventProxyBlocking, Event, EventBlocking};

pub trait EventExtError: crate::event::Event {
	type Error: std::error::Error;
}

pub trait EventExt {
}

impl<T: EventExtError + crate::event::Event> EventExt for T {
}

assert_impl_all!(EventProxy: Event, EventExt);
assert_impl_all!(EventProxyBlocking: EventBlocking, EventBlockingExt);
