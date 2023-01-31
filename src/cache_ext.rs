pub trait CacheExtError: crate::cache::Cache {
	type Error: std::error::Error;
}

pub trait CacheExt {
}

impl<T: CacheExtError + crate::cache::Cache> CacheExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	cache_ext::CacheExt,
	cache::{CacheProxy,
	CacheProxyBlocking}};	fn implements_cache_ext<T: CacheExt>() {}
	#[test]
	fn check_cache_implements_cache_ext() {
		implements_cache_ext::<CacheProxy<'static>>();
	}
	#[test]
	fn check_blocking_cache_implements_cache_ext() {
		implements_cache_ext::<CacheProxyBlocking<'static>>();
	}
}