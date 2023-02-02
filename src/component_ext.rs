pub trait ComponentExtError: crate::component::Component {
	type Error: std::error::Error;
}
pub trait ComponentBlockingExtError: crate::component::ComponentBlocking {
	type Error: std::error::Error;
}

pub trait ComponentExt {
}
pub trait ComponentBlockingExt {
}

impl<T: ComponentExtError + crate::component::Component> ComponentExt for T {
}
impl<T: ComponentBlockingExtError + crate::component::ComponentBlocking> ComponentBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    component_ext::{
      ComponentExt,
      ComponentBlockingExt,
    },
    component::{
      ComponentProxy,
      ComponentProxyBlocking,
    },
  };
  fn implements_component_ext<T: ComponentExt>() {}
  fn implements_component_blocking_ext<T: ComponentBlockingExt>() {}
	#[test]
	fn check_component_implements_component_ext() {
		implements_component_ext::<ComponentProxy<'static>>();
	}
	#[test]
	fn check_blocking_component_implements_component_ext() {
		implements_component_blocking_ext::<ComponentProxyBlocking<'static>>();
	}
}
