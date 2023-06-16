use atspi_proxies::document::{Document, DocumentBlocking, DocumentProxy, DocumentProxyBlocking};

impl_extended_errors!(DocumentProxy<'_>, DocumentExtError);
impl_extended_errors!(DocumentProxyBlocking<'_>, DocumentBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait DocumentExtError: Document {
	type Error: std::error::Error;
}
pub trait DocumentBlockingExtError: DocumentBlocking {
	type Error: std::error::Error;
}

pub trait DocumentExt {}
pub trait DocumentBlockingExt {}

impl<T: DocumentExtError + Document> DocumentExt for T {}
impl<T: DocumentBlockingExtError + DocumentBlocking> DocumentBlockingExt for T {}

assert_impl_all!(DocumentProxy: Document, DocumentExt);
assert_impl_all!(DocumentProxyBlocking: DocumentBlocking, DocumentBlockingExt);
