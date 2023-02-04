#[allow(clippy::module_name_repetitions)]
pub trait EditableTextExtError: crate::editable_text::EditableText {
	type Error: std::error::Error;
}
pub trait EditableTextBlockingExtError: crate::editable_text::EditableTextBlocking {
	type Error: std::error::Error;
}

pub trait EditableTextExt {
}
pub trait EditableTextBlockingExt {
}

impl<T: EditableTextExtError + crate::editable_text::EditableText> EditableTextExt for T {
}
impl<T: EditableTextBlockingExtError + crate::editable_text::EditableTextBlocking> EditableTextBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    editable_text_ext::{
      EditableTextExt,
      EditableTextBlockingExt,
    },
    editable_text::{
      EditableTextProxy,
      EditableTextProxyBlocking,
    },
  };
  fn implements_editable_text_ext<T: EditableTextExt>() {}
  fn implements_editable_text_blocking_ext<T: EditableTextBlockingExt>() {}
	#[test]
	fn check_editable_text_implements_editable_text_ext() {
		implements_editable_text_ext::<EditableTextProxy<'static>>();
	}
	#[test]
	fn check_blocking_editable_text_implements_editable_text_ext() {
		implements_editable_text_blocking_ext::<EditableTextProxyBlocking<'static>>();
	}
}
