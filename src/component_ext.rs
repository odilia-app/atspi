pub trait ComponentExtError: crate::component::Component {
	type Error: std::error::Error;
}

pub trait ComponentExt {
}

impl<T: ComponentExtError + crate::component::Component> ComponentExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	component_ext::ComponentExt,
	component::{ComponentProxy,
	ComponentProxyBlocking}};	fn implements_component_ext<T: ComponentExt>() {}
	#[test]
	fn check_component_implements_component_ext() {
		implements_component_ext::<ComponentProxy<'static>>();
	}
	#[test]
	fn check_blocking_component_implements_component_ext() {
		implements_component_ext::<ComponentProxyBlocking<'static>>();
	}
}