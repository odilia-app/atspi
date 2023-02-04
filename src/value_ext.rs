#[allow(clippy::module_name_repetitions)]
pub trait ValueExtError: crate::value::Value {
	type Error: std::error::Error;
}

#[allow(clippy::module_name_repetitions)]
pub trait ValueBlockingExtError: crate::value::ValueBlocking {
	type Error: std::error::Error;
}

pub trait ValueExt {
}
pub trait ValueBlockingExt {
}

impl<T: ValueExtError + crate::value::Value> ValueExt for T {
}
impl<T: ValueBlockingExtError + crate::value::ValueBlocking> ValueBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    value_ext::{
      ValueExt,
      ValueBlockingExt,
    },
    value::{
      ValueProxy,
      ValueProxyBlocking,
    },
  };
  fn implements_value_ext<T: ValueExt>() {}
  fn implements_value_blocking_ext<T: ValueBlockingExt>() {}
	#[test]
	fn check_value_implements_value_ext() {
		implements_value_ext::<ValueProxy<'static>>();
	}
	#[test]
	fn check_blocking_value_implements_value_ext() {
		implements_value_blocking_ext::<ValueProxyBlocking<'static>>();
	}
}
