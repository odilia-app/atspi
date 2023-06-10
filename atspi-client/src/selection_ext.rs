use atspi_proxies::selection::{Selection, SelectionBlocking, SelectionProxy, SelectionProxyBlocking};

impl_extended_errors!(SelectionProxy<'_>, SelectionExtError);
impl_extended_errors!(SelectionProxyBlocking<'_>, SelectionBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait SelectionExtError: Selection {
	type Error: std::error::Error;
}
pub trait SelectionBlockingExtError: SelectionBlocking {
	type Error: std::error::Error;
}

pub trait SelectionExt {}
pub trait SelectionBlockingExt {}

impl<T: SelectionExtError + Selection> SelectionExt for T {}
impl<T: SelectionBlockingExtError + SelectionBlocking> SelectionBlockingExt
	for T
{
}

assert_impl_all!(SelectionProxy: Selection, SelectionExt);
assert_impl_all!(SelectionProxyBlocking: SelectionBlocking, SelectionBlockingExt);
