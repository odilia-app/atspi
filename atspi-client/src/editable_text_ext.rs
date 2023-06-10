use atspi_proxies::editable_text::{
	EditableText, EditableTextBlocking, EditableTextProxy, EditableTextProxyBlocking,
};

impl_extended_errors!(EditableTextProxy<'_>, EditableTextExtError);
impl_extended_errors!(EditableTextProxyBlocking<'_>, EditableTextBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait EditableTextExtError: EditableText {
	type Error: std::error::Error;
}
pub trait EditableTextBlockingExtError: EditableTextBlocking {
	type Error: std::error::Error;
}

pub trait EditableTextExt {}
pub trait EditableTextBlockingExt {}

impl<T: EditableTextExtError + EditableText> EditableTextExt for T {}
impl<T: EditableTextBlockingExtError + EditableTextBlocking>
	EditableTextBlockingExt for T
{
}

assert_impl_all!(EditableTextProxy: EditableText, EditableTextExt);
assert_impl_all!(EditableTextProxyBlocking: EditableTextBlocking, EditableTextBlockingExt);
