pub trait RegistryExtError: crate::registry::Registry {
	type Error: std::error::Error;
}

pub trait RegistryExt {
}

impl<T: RegistryExtError + crate::registry::Registry> RegistryExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	registry_ext::RegistryExt,
	registry::{RegistryProxy,
	RegistryProxyBlocking}};	fn implements_registry_ext<T: RegistryExt>() {}
	#[test]
	fn check_registry_implements_registry_ext() {
		implements_registry_ext::<RegistryProxy<'static>>();
	}
	#[test]
	fn check_blocking_registry_implements_registry_ext() {
		implements_registry_ext::<RegistryProxyBlocking<'static>>();
	}
}