pub trait CollectionExtError: crate::collection::Collection {
	type Error: std::error::Error;
}
pub trait CollectionBlockingExtError: crate::collection::CollectionBlocking {
	type Error: std::error::Error;
}

pub trait CollectionExt {
}
pub trait CollectionBlockingExt {
}

impl<T: CollectionExtError + crate::collection::Collection> CollectionExt for T {
}
impl<T: CollectionBlockingExtError + crate::collection::CollectionBlocking> CollectionBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    collection_ext::{
      CollectionExt,
      CollectionBlockingExt,
    },
    collection::{
      CollectionProxy,
      CollectionProxyBlocking,
    },
  };
  fn implements_collection_ext<T: CollectionExt>() {}
  fn implements_collection_blocking_ext<T: CollectionBlockingExt>() {}
	#[test]
	fn check_collection_implements_collection_ext() {
		implements_collection_ext::<CollectionProxy<'static>>();
	}
	#[test]
	fn check_blocking_collection_implements_collection_ext() {
		implements_collection_blocking_ext::<CollectionProxyBlocking<'static>>();
	}
}
