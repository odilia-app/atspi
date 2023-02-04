#[allow(clippy::module_name_repetitions)]
pub trait DeviceEventListenerExtError: crate::device_event_listener::DeviceEventListener {
	type Error: std::error::Error;
}
pub trait DeviceEventListenerBlockingExtError: crate::device_event_listener::DeviceEventListenerBlocking {
	type Error: std::error::Error;
}

pub trait DeviceEventListenerExt {
}
pub trait DeviceEventListenerBlockingExt {
}

impl<T: DeviceEventListenerExtError + crate::device_event_listener::DeviceEventListener> DeviceEventListenerExt for T {
}
impl<T: DeviceEventListenerBlockingExtError + crate::device_event_listener::DeviceEventListenerBlocking> DeviceEventListenerBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    device_event_listener_ext::{
      DeviceEventListenerExt,
      DeviceEventListenerBlockingExt,
    },
    device_event_listener::{
      DeviceEventListenerProxy,
      DeviceEventListenerProxyBlocking
    },
  };
  fn implements_device_event_listener_ext<T: DeviceEventListenerExt>() {}
  fn implements_device_event_listener_blocking_ext<T: DeviceEventListenerBlockingExt>() {}
	#[test]
	fn check_device_event_listener_implements_device_event_listener_ext() {
		implements_device_event_listener_ext::<DeviceEventListenerProxy<'static>>();
	}
	#[test]
	fn check_blocking_device_event_listener_implements_device_event_listener_ext() {
		implements_device_event_listener_blocking_ext::<DeviceEventListenerProxyBlocking<'static>>();
	}
}
