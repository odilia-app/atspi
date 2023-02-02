pub trait HypertextExtError: crate::hypertext::Hypertext {
	type Error: std::error::Error;
}
pub trait HypertextBlockingExtError: crate::hypertext::HypertextBlocking {
	type Error: std::error::Error;
}

pub trait HypertextExt {
}
pub trait HypertextBlockingExt {
}

impl<T: HypertextExtError + crate::hypertext::Hypertext> HypertextExt for T {
}
impl<T: HypertextBlockingExtError + crate::hypertext::HypertextBlocking> HypertextBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    hypertext_ext::{
      HypertextExt,
      HypertextBlockingExt,
    },
    hypertext::{
      HypertextProxy,
	    HypertextProxyBlocking
    },
  };
  fn implements_hypertext_ext<T: HypertextExt>() {}
  fn implements_hypertext_blocking_ext<T: HypertextBlockingExt>() {}
	#[test]
	fn check_hypertext_implements_hypertext_ext() {
		implements_hypertext_ext::<HypertextProxy<'static>>();
	}
	#[test]
	fn check_blocking_hypertext_implements_hypertext_ext() {
		implements_hypertext_blocking_ext::<HypertextProxyBlocking<'static>>();
	}
}
