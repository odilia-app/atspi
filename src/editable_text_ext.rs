pub trait EditableTextExtError: crate::editable_text::EditableText {
	type Error: std::error::Error;
}

pub trait EditableTextExt {
}

impl<T: EditableTextExtError + crate::editable_text::EditableText> EditableTextExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	editable_text_ext::EditableTextExt,
	editable_text::{EditableTextProxy,
	EditableTextProxyBlocking}};	fn implements_editable_text_ext<T: EditableTextExt>() {}
	#[test]
	fn check_editable_text_implements_editable_text_ext() {
		implements_editable_text_ext::<EditableTextProxy<'static>>();
	}
	#[test]
	fn check_blocking_editable_text_implements_editable_text_ext() {
		implements_editable_text_ext::<EditableTextProxyBlocking<'static>>();
	}
}