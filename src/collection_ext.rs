pub trait CollectionExtError: crate::collection::Collection {
	type Error: std::error::Error;
}

pub trait CollectionExt {
}

impl<T: CollectionExtError + crate::collection::Collection> CollectionExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	collection_ext::CollectionExt,
	collection::{CollectionProxy,
	CollectionProxyBlocking}};	fn implements_collection_ext<T: CollectionExt>() {}
	#[test]
	fn check_collection_implements_collection_ext() {
		implements_collection_ext::<CollectionProxy<'static>>();
	}
	#[test]
	fn check_blocking_collection_implements_collection_ext() {
		implements_collection_ext::<CollectionProxyBlocking<'static>>();
	}
}