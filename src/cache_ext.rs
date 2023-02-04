#[allow(clippy::module_name_repetitions)]
pub trait CacheExtError: crate::cache::Cache {
	type Error: std::error::Error;
}
pub trait CacheBlockingExtError: crate::cache::CacheBlocking {
	type Error: std::error::Error;
}

pub trait CacheExt {
}
pub trait CacheBlockingExt {
}

impl<T: CacheExtError + crate::cache::Cache> CacheExt for T {
}
impl<T: CacheBlockingExtError + crate::cache::CacheBlocking> CacheBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    cache_ext::{
      CacheExt,
      CacheBlockingExt,
    },
    cache::{
      CacheProxy,
      CacheProxyBlocking,
    },
  };
  fn implements_cache_ext<T: CacheExt>() {}
  fn implements_cache_blocking_ext<T: CacheBlockingExt>() {}
	#[test]
	fn check_cache_implements_cache_ext() {
		implements_cache_ext::<CacheProxy<'static>>();
	}
	#[test]
	fn check_blocking_cache_implements_cache_ext() {
		implements_cache_blocking_ext::<CacheProxyBlocking<'static>>();
	}
}
