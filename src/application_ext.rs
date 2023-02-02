pub trait ApplicationExtError: crate::application::Application {
	type Error: std::error::Error;
}
pub trait ApplicationBlockingExtError: crate::application::ApplicationBlocking {
	type Error: std::error::Error;
}

pub trait ApplicationExt {
}
pub trait ApplicationBlockingExt {
}

impl<T: ApplicationExtError + crate::application::Application> ApplicationExt for T {
}
impl<T: ApplicationBlockingExtError + crate::application::ApplicationBlocking> ApplicationBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    application_ext::{
      ApplicationExt,
      ApplicationBlockingExt,
    },
    application::{
      ApplicationProxy,
      ApplicationProxyBlocking
    },
  };
  fn implements_application_ext<T: ApplicationExt>() {}
  fn implements_application_blocking_ext<T: ApplicationBlockingExt>() {}
	#[test]
	fn check_application_implements_application_ext() {
		implements_application_ext::<ApplicationProxy<'static>>();
	}
	#[test]
	fn check_blocking_application_implements_application_ext() {
		implements_application_blocking_ext::<ApplicationProxyBlocking<'static>>();
	}
}
