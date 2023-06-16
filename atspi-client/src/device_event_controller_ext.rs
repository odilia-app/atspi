use atspi_proxies::device_event_controller::{
	DeviceEventController, DeviceEventControllerBlocking, DeviceEventControllerProxy,
	DeviceEventControllerProxyBlocking,
};

impl_extended_errors!(DeviceEventControllerProxy<'_>, DeviceEventControllerExtError);
impl_extended_errors!(
	DeviceEventControllerProxyBlocking<'_>,
	DeviceEventControllerBlockingExtError
);

#[allow(clippy::module_name_repetitions)]
pub trait DeviceEventControllerExtError: DeviceEventController {
	type Error: std::error::Error;
}
pub trait DeviceEventControllerBlockingExtError: DeviceEventControllerBlocking {
	type Error: std::error::Error;
}

pub trait DeviceEventControllerExt {}
pub trait DeviceEventControllerBlockingExt {}

impl<T: DeviceEventControllerExtError + DeviceEventController> DeviceEventControllerExt for T {}
impl<T: DeviceEventControllerBlockingExtError + DeviceEventControllerBlocking>
	DeviceEventControllerBlockingExt for T
{
}

assert_impl_all!(DeviceEventControllerProxy: DeviceEventController, DeviceEventControllerExt);
assert_impl_all!(
	DeviceEventControllerProxyBlocking: DeviceEventControllerBlocking,
	DeviceEventControllerBlockingExt
);
