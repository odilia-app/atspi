use atspi_proxies::collection::{Collection, CollectionBlocking, CollectionProxy, CollectionProxyBlocking};

impl_extended_errors!(CollectionProxy<'_>, CollectionExtError);
impl_extended_errors!(CollectionProxyBlocking<'_>, CollectionBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait CollectionExtError: Collection {
	type Error: std::error::Error;
}
pub trait CollectionBlockingExtError: CollectionBlocking {
	type Error: std::error::Error;
}

pub trait CollectionExt {}
pub trait CollectionBlockingExt {}

impl<T: CollectionExtError + Collection> CollectionExt for T {}
impl<T: CollectionBlockingExtError + CollectionBlocking> CollectionBlockingExt
	for T
{
}

assert_impl_all!(CollectionProxy: Collection, CollectionExt);
assert_impl_all!(CollectionProxyBlocking: CollectionBlocking, CollectionBlockingExt);
