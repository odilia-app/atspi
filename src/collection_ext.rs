use crate::collection::{Collection, CollectionBlocking, CollectionProxy, CollectionProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait CollectionExtError: crate::collection::Collection {
	type Error: std::error::Error;
}
pub trait CollectionBlockingExtError: crate::collection::CollectionBlocking {
	type Error: std::error::Error;
}

pub trait CollectionExt {}
pub trait CollectionBlockingExt {}

impl<T: CollectionExtError + crate::collection::Collection> CollectionExt for T {}
impl<T: CollectionBlockingExtError + crate::collection::CollectionBlocking> CollectionBlockingExt
	for T
{
}

assert_impl_all!(CollectionProxy: Collection, CollectionExt);
assert_impl_all!(CollectionProxyBlocking: CollectionBlocking, CollectionBlockingExt);
