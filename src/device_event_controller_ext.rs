use crate::device_event_controller::{
	DeviceEventController, DeviceEventControllerBlocking, DeviceEventControllerProxy,
	DeviceEventControllerProxyBlocking,
};

#[allow(clippy::module_name_repetitions)]
pub trait DeviceEventControllerExtError:
	crate::device_event_controller::DeviceEventController
{
	type Error: std::error::Error;
}
pub trait DeviceEventControllerBlockingExtError:
	crate::device_event_controller::DeviceEventControllerBlocking
{
	type Error: std::error::Error;
}

pub trait DeviceEventControllerExt {}
pub trait DeviceEventControllerBlockingExt {}

impl<T: DeviceEventControllerExtError + crate::device_event_controller::DeviceEventController>
	DeviceEventControllerExt for T
{
}
impl<
		T: DeviceEventControllerBlockingExtError
			+ crate::device_event_controller::DeviceEventControllerBlocking,
	> DeviceEventControllerBlockingExt for T
{
}

assert_impl_all!(DeviceEventControllerProxy: DeviceEventController, DeviceEventControllerExt);
assert_impl_all!(
	DeviceEventControllerProxyBlocking: DeviceEventControllerBlocking,
	DeviceEventControllerBlockingExt
);
