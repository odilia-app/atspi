use crate::hyperlink::{Hyperlink, HyperlinkBlocking, HyperlinkProxy, HyperlinkProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait HyperlinkExtError: crate::hyperlink::Hyperlink {
	type Error: std::error::Error;
}
pub trait HyperlinkBlockingExtError: crate::hyperlink::HyperlinkBlocking {
	type Error: std::error::Error;
}

pub trait HyperlinkExt {}
pub trait HyperlinkBlockingExt {}

impl<T: HyperlinkExtError + crate::hyperlink::Hyperlink> HyperlinkExt for T {}
impl<T: HyperlinkBlockingExtError + crate::hyperlink::HyperlinkBlocking> HyperlinkBlockingExt
	for T
{
}

assert_impl_all!(HyperlinkProxy: Hyperlink, HyperlinkExt);
assert_impl_all!(HyperlinkProxyBlocking: HyperlinkBlocking, HyperlinkBlockingExt);
