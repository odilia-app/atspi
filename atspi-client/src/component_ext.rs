use atspi_proxies::component::{
	Component, ComponentBlocking, ComponentProxy, ComponentProxyBlocking,
};

impl_extended_errors!(ComponentProxy<'_>, ComponentExtError);
impl_extended_errors!(ComponentProxyBlocking<'_>, ComponentBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait ComponentExtError: Component {
	type Error: std::error::Error;
}
pub trait ComponentBlockingExtError: ComponentBlocking {
	type Error: std::error::Error;
}

pub trait ComponentExt {}
pub trait ComponentBlockingExt {}

impl<T: ComponentExtError + Component> ComponentExt for T {}
impl<T: ComponentBlockingExtError + ComponentBlocking> ComponentBlockingExt for T {}

assert_impl_all!(ComponentProxy: Component, ComponentExt);
assert_impl_all!(ComponentProxyBlocking: ComponentBlocking, ComponentBlockingExt);
