use crate::registry::{Registry, RegistryBlocking, RegistryProxy, RegistryProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait RegistryExtError: crate::registry::Registry {
	type Error: std::error::Error;
}
pub trait RegistryBlockingExtError: crate::registry::RegistryBlocking {
	type Error: std::error::Error;
}

pub trait RegistryExt {}
pub trait RegistryBlockingExt {}

impl<T: RegistryExtError + crate::registry::Registry> RegistryExt for T {}
impl<T: RegistryBlockingExtError + crate::registry::RegistryBlocking> RegistryBlockingExt for T {}

assert_impl_all!(RegistryProxy: Registry, RegistryExt);
assert_impl_all!(RegistryProxyBlocking: RegistryBlocking, RegistryBlockingExt);
