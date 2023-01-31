pub trait ApplicationExtError: crate::application::Application {
	type Error: std::error::Error;
}

pub trait ApplicationExt {
}

impl<T: ApplicationExtError + crate::application::Application> ApplicationExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	application_ext::ApplicationExt,
	application::{ApplicationProxy,
	ApplicationProxyBlocking}};	fn implements_application_ext<T: ApplicationExt>() {}
	#[test]
	fn check_application_implements_application_ext() {
		implements_application_ext::<ApplicationProxy<'static>>();
	}
	#[test]
	fn check_blocking_application_implements_application_ext() {
		implements_application_ext::<ApplicationProxyBlocking<'static>>();
	}
}