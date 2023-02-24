use crate::cache::{Cache, CacheBlocking, CacheProxy, CacheProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait CacheExtError: crate::cache::Cache {
	type Error: std::error::Error;
}
pub trait CacheBlockingExtError: crate::cache::CacheBlocking {
	type Error: std::error::Error;
}

pub trait CacheExt {}
pub trait CacheBlockingExt {}

impl<T: CacheExtError + crate::cache::Cache> CacheExt for T {}
impl<T: CacheBlockingExtError + crate::cache::CacheBlocking> CacheBlockingExt for T {}

assert_impl_all!(CacheProxy: Cache, CacheExt);
assert_impl_all!(CacheProxyBlocking: CacheBlocking, CacheBlockingExt);
