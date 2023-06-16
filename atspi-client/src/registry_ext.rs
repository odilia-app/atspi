use atspi_proxies::registry::{Registry, RegistryBlocking, RegistryProxy, RegistryProxyBlocking};

impl_extended_errors!(RegistryProxy<'_>, RegistryExtError);
impl_extended_errors!(RegistryProxyBlocking<'_>, RegistryBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait RegistryExtError: Registry {
	type Error: std::error::Error;
}
pub trait RegistryBlockingExtError: RegistryBlocking {
	type Error: std::error::Error;
}

pub trait RegistryExt {}
pub trait RegistryBlockingExt {}

impl<T: RegistryExtError + Registry> RegistryExt for T {}
impl<T: RegistryBlockingExtError + RegistryBlocking> RegistryBlockingExt for T {}

assert_impl_all!(RegistryProxy: Registry, RegistryExt);
assert_impl_all!(RegistryProxyBlocking: RegistryBlocking, RegistryBlockingExt);
