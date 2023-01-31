pub trait ValueExtError: crate::value::Value {
	type Error: std::error::Error;
}

pub trait ValueExt {
}

impl<T: ValueExtError + crate::value::Value> ValueExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	value_ext::ValueExt,
	value::{ValueProxy,
	ValueProxyBlocking}};	fn implements_value_ext<T: ValueExt>() {}
	#[test]
	fn check_value_implements_value_ext() {
		implements_value_ext::<ValueProxy<'static>>();
	}
	#[test]
	fn check_blocking_value_implements_value_ext() {
		implements_value_ext::<ValueProxyBlocking<'static>>();
	}
}
