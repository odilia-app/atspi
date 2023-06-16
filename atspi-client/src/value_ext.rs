use atspi_proxies::value::{Value, ValueBlocking, ValueProxy, ValueProxyBlocking};

impl_extended_errors!(ValueProxy<'_>, ValueExtError);
impl_extended_errors!(ValueProxyBlocking<'_>, ValueBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait ValueExtError: Value {
	type Error: std::error::Error;
}

#[allow(clippy::module_name_repetitions)]
pub trait ValueBlockingExtError: ValueBlocking {
	type Error: std::error::Error;
}

pub trait ValueExt {}
pub trait ValueBlockingExt {}

impl<T: ValueExtError + Value> ValueExt for T {}
impl<T: ValueBlockingExtError + ValueBlocking> ValueBlockingExt for T {}

assert_impl_all!(ValueProxy: Value, ValueExt);
assert_impl_all!(ValueProxyBlocking: ValueBlocking, ValueBlockingExt);
