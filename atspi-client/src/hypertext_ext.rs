use atspi_proxies::hypertext::{Hypertext, HypertextBlocking, HypertextProxy, HypertextProxyBlocking};

impl_extended_errors!(HypertextProxy<'_>, HypertextExtError);
impl_extended_errors!(HypertextProxyBlocking<'_>, HypertextBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait HypertextExtError: Hypertext {
	type Error: std::error::Error;
}
pub trait HypertextBlockingExtError: HypertextBlocking {
	type Error: std::error::Error;
}

pub trait HypertextExt {}
pub trait HypertextBlockingExt {}

impl<T: HypertextExtError + Hypertext> HypertextExt for T {}
impl<T: HypertextBlockingExtError + HypertextBlocking> HypertextBlockingExt
	for T
{
}

assert_impl_all!(HypertextProxy: Hypertext, HypertextExt);
assert_impl_all!(HypertextProxyBlocking: HypertextBlocking, HypertextBlockingExt);
