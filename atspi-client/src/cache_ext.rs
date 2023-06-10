use atspi_proxies::cache::{Cache, CacheBlocking, CacheProxy, CacheProxyBlocking};

impl_extended_errors!(CacheProxy<'_>, CacheExtError);
impl_extended_errors!(CacheProxyBlocking<'_>, CacheBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait CacheExtError: Cache {
	type Error: std::error::Error;
}
pub trait CacheBlockingExtError: CacheBlocking {
	type Error: std::error::Error;
}

pub trait CacheExt {}
pub trait CacheBlockingExt {}

impl<T: CacheExtError + Cache> CacheExt for T {}
impl<T: CacheBlockingExtError + CacheBlocking> CacheBlockingExt for T {}

assert_impl_all!(CacheProxy: Cache, CacheExt);
assert_impl_all!(CacheProxyBlocking: CacheBlocking, CacheBlockingExt);
