#[allow(clippy::module_name_repetitions)]
pub trait HyperlinkExtError: crate::hyperlink::Hyperlink {
	type Error: std::error::Error;
}
pub trait HyperlinkBlockingExtError: crate::hyperlink::HyperlinkBlocking {
	type Error: std::error::Error;
}

pub trait HyperlinkExt {
}
pub trait HyperlinkBlockingExt {
}

impl<T: HyperlinkExtError + crate::hyperlink::Hyperlink> HyperlinkExt for T {
}
impl<T: HyperlinkBlockingExtError + crate::hyperlink::HyperlinkBlocking> HyperlinkBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    hyperlink_ext::{
      HyperlinkExt,
      HyperlinkBlockingExt,
    },
    hyperlink::{
      HyperlinkProxy,
	    HyperlinkProxyBlocking
    }
  };
  fn implements_hyperlink_ext<T: HyperlinkExt>() {}
  fn implements_hyperlink_blocking_ext<T: HyperlinkBlockingExt>() {}
	#[test]
	fn check_hyperlink_implements_hyperlink_ext() {
		implements_hyperlink_ext::<HyperlinkProxy<'static>>();
	}
	#[test]
	fn check_blocking_hyperlink_implements_hyperlink_ext() {
		implements_hyperlink_blocking_ext::<HyperlinkProxyBlocking<'static>>();
	}
}
