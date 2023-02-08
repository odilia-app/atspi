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

#[cfg(test)]
mod test {
	use crate::{
		document::{DocumentProxy, DocumentProxyBlocking},
		document_ext::{DocumentBlockingExt, DocumentExt},
	};
	fn implements_document_ext<T: DocumentExt>() {}
	fn implements_document_blocking_ext<T: DocumentBlockingExt>() {}
	#[test]
	fn check_document_implements_document_ext() {
		implements_document_ext::<DocumentProxy<'static>>();
	}
	#[test]
	fn check_blocking_document_implements_document_ext() {
		implements_document_blocking_ext::<DocumentProxyBlocking<'static>>();
	}
}
