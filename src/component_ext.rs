use crate::component::{Component, ComponentBlocking, ComponentProxy, ComponentProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait ComponentExtError: crate::component::Component {
	type Error: std::error::Error;
}
pub trait ComponentBlockingExtError: crate::component::ComponentBlocking {
	type Error: std::error::Error;
}

pub trait ComponentExt {}
pub trait ComponentBlockingExt {}

impl<T: ComponentExtError + crate::component::Component> ComponentExt for T {}
impl<T: ComponentBlockingExtError + crate::component::ComponentBlocking> ComponentBlockingExt
	for T
{
}

assert_impl_all!(ComponentProxy: Component, ComponentExt);
assert_impl_all!(ComponentProxyBlocking: ComponentBlocking, ComponentBlockingExt);
