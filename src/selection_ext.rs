use crate::selection::{Selection, SelectionBlocking, SelectionProxy, SelectionProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait SelectionExtError: crate::selection::Selection {
	type Error: std::error::Error;
}
pub trait SelectionBlockingExtError: crate::selection::SelectionBlocking {
	type Error: std::error::Error;
}

pub trait SelectionExt {}
pub trait SelectionBlockingExt {}

impl<T: SelectionExtError + crate::selection::Selection> SelectionExt for T {}
impl<T: SelectionBlockingExtError + crate::selection::SelectionBlocking> SelectionBlockingExt
	for T
{
}

assert_impl_all!(SelectionProxy: Selection, SelectionExt);
assert_impl_all!(SelectionProxyBlocking: SelectionBlocking, SelectionBlockingExt);
