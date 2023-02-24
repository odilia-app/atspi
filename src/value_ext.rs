use crate::value::{Value, ValueBlocking, ValueProxy, ValueProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait ValueExtError: crate::value::Value {
	type Error: std::error::Error;
}

#[allow(clippy::module_name_repetitions)]
pub trait ValueBlockingExtError: crate::value::ValueBlocking {
	type Error: std::error::Error;
}

pub trait ValueExt {}
pub trait ValueBlockingExt {}

impl<T: ValueExtError + crate::value::Value> ValueExt for T {}
impl<T: ValueBlockingExtError + crate::value::ValueBlocking> ValueBlockingExt for T {}

assert_impl_all!(ValueProxy: Value, ValueExt);
assert_impl_all!(ValueProxyBlocking: ValueBlocking, ValueBlockingExt);
