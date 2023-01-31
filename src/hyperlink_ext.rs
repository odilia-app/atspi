pub trait HyperlinkExtError: crate::hyperlink::Hyperlink {
	type Error: std::error::Error;
}

pub trait HyperlinkExt {
}

impl<T: HyperlinkExtError + crate::hyperlink::Hyperlink> HyperlinkExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	hyperlink_ext::HyperlinkExt,
	hyperlink::{HyperlinkProxy,
	HyperlinkProxyBlocking}};	fn implements_hyperlink_ext<T: HyperlinkExt>() {}
	#[test]
	fn check_hyperlink_implements_hyperlink_ext() {
		implements_hyperlink_ext::<HyperlinkProxy<'static>>();
	}
	#[test]
	fn check_blocking_hyperlink_implements_hyperlink_ext() {
		implements_hyperlink_ext::<HyperlinkProxyBlocking<'static>>();
	}
}