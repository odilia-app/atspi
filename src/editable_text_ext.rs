use crate::editable_text::{
	EditableText, EditableTextBlocking, EditableTextProxy, EditableTextProxyBlocking,
};

#[allow(clippy::module_name_repetitions)]
pub trait EditableTextExtError: crate::editable_text::EditableText {
	type Error: std::error::Error;
}
pub trait EditableTextBlockingExtError: crate::editable_text::EditableTextBlocking {
	type Error: std::error::Error;
}

pub trait EditableTextExt {}
pub trait EditableTextBlockingExt {}

impl<T: EditableTextExtError + crate::editable_text::EditableText> EditableTextExt for T {}
impl<T: EditableTextBlockingExtError + crate::editable_text::EditableTextBlocking>
	EditableTextBlockingExt for T
{
}

assert_impl_all!(EditableTextProxy: EditableText, EditableTextExt);
assert_impl_all!(EditableTextProxyBlocking: EditableTextBlocking, EditableTextBlockingExt);
