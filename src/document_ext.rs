use crate::document::{Document, DocumentBlocking, DocumentProxy, DocumentProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait DocumentExtError: crate::document::Document {
	type Error: std::error::Error;
}
pub trait DocumentBlockingExtError: crate::document::DocumentBlocking {
	type Error: std::error::Error;
}

pub trait DocumentExt {}
pub trait DocumentBlockingExt {}

impl<T: DocumentExtError + crate::document::Document> DocumentExt for T {}
impl<T: DocumentBlockingExtError + crate::document::DocumentBlocking> DocumentBlockingExt for T {}

assert_impl_all!(DocumentProxy: Document, DocumentExt);
assert_impl_all!(DocumentProxyBlocking: DocumentBlocking, DocumentBlockingExt);
