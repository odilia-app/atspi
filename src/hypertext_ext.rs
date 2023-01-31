pub trait HypertextExtError: crate::hypertext::Hypertext {
	type Error: std::error::Error;
}

pub trait HypertextExt {
}

impl<T: HypertextExtError + crate::hypertext::Hypertext> HypertextExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	hypertext_ext::HypertextExt,
	hypertext::{HypertextProxy,
	HypertextProxyBlocking}};	fn implements_hypertext_ext<T: HypertextExt>() {}
	#[test]
	fn check_hypertext_implements_hypertext_ext() {
		implements_hypertext_ext::<HypertextProxy<'static>>();
	}
	#[test]
	fn check_blocking_hypertext_implements_hypertext_ext() {
		implements_hypertext_ext::<HypertextProxyBlocking<'static>>();
	}
}