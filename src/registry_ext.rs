pub trait RegistryExtError: crate::registry::Registry {
	type Error: std::error::Error;
}
pub trait RegistryBlockingExtError: crate::registry::RegistryBlocking {
	type Error: std::error::Error;
}

pub trait RegistryExt {
}
pub trait RegistryBlockingExt {
}

impl<T: RegistryExtError + crate::registry::Registry> RegistryExt for T {
}
impl<T: RegistryBlockingExtError + crate::registry::RegistryBlocking> RegistryBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    registry_ext::{
      RegistryExt,
      RegistryBlockingExt,
    },
    registry::{
      RegistryProxy,
	    RegistryProxyBlocking,
    },
  };
  fn implements_registry_ext<T: RegistryExt>() {}
  fn implements_registry_blocking_ext<T: RegistryBlockingExt>() {}
	#[test]
	fn check_registry_implements_registry_ext() {
		implements_registry_ext::<RegistryProxy<'static>>();
	}
	#[test]
	fn check_blocking_registry_implements_registry_ext() {
		implements_registry_blocking_ext::<RegistryProxyBlocking<'static>>();
	}
}
