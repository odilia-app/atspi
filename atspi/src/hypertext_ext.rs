use crate::hypertext::{Hypertext, HypertextBlocking, HypertextProxy, HypertextProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait HypertextExtError: crate::hypertext::Hypertext {
	type Error: std::error::Error;
}
pub trait HypertextBlockingExtError: crate::hypertext::HypertextBlocking {
	type Error: std::error::Error;
}

pub trait HypertextExt {}
pub trait HypertextBlockingExt {}

impl<T: HypertextExtError + crate::hypertext::Hypertext> HypertextExt for T {}
impl<T: HypertextBlockingExtError + crate::hypertext::HypertextBlocking> HypertextBlockingExt
	for T
{
}

assert_impl_all!(HypertextProxy: Hypertext, HypertextExt);
assert_impl_all!(HypertextProxyBlocking: HypertextBlocking, HypertextBlockingExt);
