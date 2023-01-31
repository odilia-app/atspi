pub trait DocumentExtError: crate::document::Document {
	type Error: std::error::Error;
}

pub trait DocumentExt {
}

impl<T: DocumentExtError + crate::document::Document> DocumentExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	document_ext::DocumentExt,
	document::{DocumentProxy,
	DocumentProxyBlocking}};	fn implements_document_ext<T: DocumentExt>() {}
	#[test]
	fn check_document_implements_document_ext() {
		implements_document_ext::<DocumentProxy<'static>>();
	}
	#[test]
	fn check_blocking_document_implements_document_ext() {
		implements_document_ext::<DocumentProxyBlocking<'static>>();
	}
}