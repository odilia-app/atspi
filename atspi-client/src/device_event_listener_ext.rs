use atspi_proxies::device_event_listener::{
	DeviceEventListener, DeviceEventListenerBlocking, DeviceEventListenerProxy,
	DeviceEventListenerProxyBlocking,
};

impl_extended_errors!(DeviceEventListenerProxy<'_>, DeviceEventListenerExtError);
impl_extended_errors!(DeviceEventListenerProxyBlocking<'_>, DeviceEventListenerBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait DeviceEventListenerExtError: DeviceEventListener {
	type Error: std::error::Error;
}
pub trait DeviceEventListenerBlockingExtError:
	DeviceEventListenerBlocking
{
	type Error: std::error::Error;
}

pub trait DeviceEventListenerExt {}
pub trait DeviceEventListenerBlockingExt {}

impl<T: DeviceEventListenerExtError + DeviceEventListener>
	DeviceEventListenerExt for T
{
}
impl<
		T: DeviceEventListenerBlockingExtError
			+ DeviceEventListenerBlocking,
	> DeviceEventListenerBlockingExt for T
{
}

assert_impl_all!(DeviceEventListenerProxy: DeviceEventListener, DeviceEventListenerExt);
assert_impl_all!(
	DeviceEventListenerProxyBlocking: DeviceEventListenerBlocking,
	DeviceEventListenerBlockingExt
);
