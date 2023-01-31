pub trait DeviceEventListenerExtError: crate::device_event_listener::DeviceEventListener {
	type Error: std::error::Error;
}

pub trait DeviceEventListenerExt {
}

impl<T: DeviceEventListenerExtError + crate::device_event_listener::DeviceEventListener> DeviceEventListenerExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	device_event_listener_ext::DeviceEventListenerExt,
	device_event_listener::{DeviceEventListenerProxy,
	DeviceEventListenerProxyBlocking}};	fn implements_device_event_listener_ext<T: DeviceEventListenerExt>() {}
	#[test]
	fn check_device_event_listener_implements_device_event_listener_ext() {
		implements_device_event_listener_ext::<DeviceEventListenerProxy<'static>>();
	}
	#[test]
	fn check_blocking_device_event_listener_implements_device_event_listener_ext() {
		implements_device_event_listener_ext::<DeviceEventListenerProxyBlocking<'static>>();
	}
}