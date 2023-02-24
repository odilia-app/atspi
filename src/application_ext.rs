use crate::application::{
	Application, ApplicationBlocking, ApplicationProxy, ApplicationProxyBlocking,
};

#[allow(clippy::module_name_repetitions)]
pub trait ApplicationExtError: crate::application::Application {
	type Error: std::error::Error;
}
pub trait ApplicationBlockingExtError: crate::application::ApplicationBlocking {
	type Error: std::error::Error;
}

pub trait ApplicationExt {}
pub trait ApplicationBlockingExt {}

impl<T: ApplicationExtError + crate::application::Application> ApplicationExt for T {}
impl<T: ApplicationBlockingExtError + crate::application::ApplicationBlocking>
	ApplicationBlockingExt for T
{
}

assert_impl_all!(ApplicationProxy: Application, ApplicationExt);
assert_impl_all!(ApplicationProxyBlocking: ApplicationBlocking, ApplicationBlockingExt);
