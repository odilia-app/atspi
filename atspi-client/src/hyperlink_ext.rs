use atspi_proxies::hyperlink::{
	Hyperlink, HyperlinkBlocking, HyperlinkProxy, HyperlinkProxyBlocking,
};

impl_extended_errors!(HyperlinkProxy<'_>, HyperlinkExtError);
impl_extended_errors!(HyperlinkProxyBlocking<'_>, HyperlinkBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait HyperlinkExtError: Hyperlink {
	type Error: std::error::Error;
}
pub trait HyperlinkBlockingExtError: HyperlinkBlocking {
	type Error: std::error::Error;
}

pub trait HyperlinkExt {}
pub trait HyperlinkBlockingExt {}

impl<T: HyperlinkExtError + Hyperlink> HyperlinkExt for T {}
impl<T: HyperlinkBlockingExtError + HyperlinkBlocking> HyperlinkBlockingExt for T {}

assert_impl_all!(HyperlinkProxy: Hyperlink, HyperlinkExt);
assert_impl_all!(HyperlinkProxyBlocking: HyperlinkBlocking, HyperlinkBlockingExt);
