use crate::device_event_listener::{
	DeviceEventListener, DeviceEventListenerBlocking, DeviceEventListenerProxy,
	DeviceEventListenerProxyBlocking,
};

#[allow(clippy::module_name_repetitions)]
pub trait DeviceEventListenerExtError: crate::device_event_listener::DeviceEventListener {
	type Error: std::error::Error;
}
pub trait DeviceEventListenerBlockingExtError:
	crate::device_event_listener::DeviceEventListenerBlocking
{
	type Error: std::error::Error;
}

pub trait DeviceEventListenerExt {}
pub trait DeviceEventListenerBlockingExt {}

impl<T: DeviceEventListenerExtError + crate::device_event_listener::DeviceEventListener>
	DeviceEventListenerExt for T
{
}
impl<
		T: DeviceEventListenerBlockingExtError
			+ crate::device_event_listener::DeviceEventListenerBlocking,
	> DeviceEventListenerBlockingExt for T
{
}

assert_impl_all!(DeviceEventListenerProxy: DeviceEventListener, DeviceEventListenerExt);
assert_impl_all!(
	DeviceEventListenerProxyBlocking: DeviceEventListenerBlocking,
	DeviceEventListenerBlockingExt
);
