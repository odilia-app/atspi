pub trait DeviceEventControllerExtError: crate::device_event_controller::DeviceEventController {
	type Error: std::error::Error;
}

pub trait DeviceEventControllerExt {
}

impl<T: DeviceEventControllerExtError + crate::device_event_controller::DeviceEventController> DeviceEventControllerExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	device_event_controller_ext::DeviceEventControllerExt,
	device_event_controller::{DeviceEventControllerProxy,
	DeviceEventControllerProxyBlocking}};	fn implements_device_event_controller_ext<T: DeviceEventControllerExt>() {}
	#[test]
	fn check_device_event_controller_implements_device_event_controller_ext() {
		implements_device_event_controller_ext::<DeviceEventControllerProxy<'static>>();
	}
	#[test]
	fn check_blocking_device_event_controller_implements_device_event_controller_ext() {
		implements_device_event_controller_ext::<DeviceEventControllerProxyBlocking<'static>>();
	}
}