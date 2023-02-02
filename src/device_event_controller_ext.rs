pub trait DeviceEventControllerExtError: crate::device_event_controller::DeviceEventController {
	type Error: std::error::Error;
}
pub trait DeviceEventControllerBlockingExtError: crate::device_event_controller::DeviceEventControllerBlocking {
	type Error: std::error::Error;
}

pub trait DeviceEventControllerExt {
}
pub trait DeviceEventControllerBlockingExt {
}

impl<T: DeviceEventControllerExtError + crate::device_event_controller::DeviceEventController> DeviceEventControllerExt for T {
}
impl<T: DeviceEventControllerBlockingExtError + crate::device_event_controller::DeviceEventControllerBlocking> DeviceEventControllerBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    device_event_controller_ext::{
      DeviceEventControllerExt,
      DeviceEventControllerBlockingExt,
    },
    device_event_controller::{
      DeviceEventControllerProxy,
      DeviceEventControllerProxyBlocking,
    },
  };
  fn implements_device_event_controller_ext<T: DeviceEventControllerExt>() {}
  fn implements_device_event_controller_blocking_ext<T: DeviceEventControllerBlockingExt>() {}
	#[test]
	fn check_device_event_controller_implements_device_event_controller_ext() {
		implements_device_event_controller_ext::<DeviceEventControllerProxy<'static>>();
	}
	#[test]
	fn check_blocking_device_event_controller_implements_device_event_controller_ext() {
		implements_device_event_controller_blocking_ext::<DeviceEventControllerProxyBlocking<'static>>();
	}
}
