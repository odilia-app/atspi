use atspi_proxies::application::{
	Application, ApplicationBlocking, ApplicationProxy, ApplicationProxyBlocking,
};

impl_extended_errors!(ApplicationProxy<'_>, ApplicationExtError);
impl_extended_errors!(ApplicationProxyBlocking<'_>, ApplicationBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait ApplicationExtError: Application {
	type Error: std::error::Error;
}
pub trait ApplicationBlockingExtError: ApplicationBlocking {
	type Error: std::error::Error;
}

pub trait ApplicationExt {}
pub trait ApplicationBlockingExt {}

impl<T: ApplicationExtError + Application> ApplicationExt for T {}
impl<T: ApplicationBlockingExtError + ApplicationBlocking> ApplicationBlockingExt for T {}

assert_impl_all!(ApplicationProxy: Application, ApplicationExt);
assert_impl_all!(ApplicationProxyBlocking: ApplicationBlocking, ApplicationBlockingExt);
