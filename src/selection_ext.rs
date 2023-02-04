#[allow(clippy::module_name_repetitions)]
pub trait SelectionExtError: crate::selection::Selection {
	type Error: std::error::Error;
}
pub trait SelectionBlockingExtError: crate::selection::SelectionBlocking {
	type Error: std::error::Error;
}

pub trait SelectionExt {
}
pub trait SelectionBlockingExt {
}

impl<T: SelectionExtError + crate::selection::Selection> SelectionExt for T {
}
impl<T: SelectionBlockingExtError + crate::selection::SelectionBlocking> SelectionBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    selection_ext::{
      SelectionExt,
      SelectionBlockingExt,
    },
    selection::{
      SelectionProxy,
	    SelectionProxyBlocking
    }
  };
  fn implements_selection_ext<T: SelectionExt>() {}
  fn implements_selection_blocking_ext<T: SelectionBlockingExt>() {}
	#[test]
	fn check_selection_implements_selection_ext() {
		implements_selection_ext::<SelectionProxy<'static>>();
	}
	#[test]
	fn check_blocking_selection_implements_selection_ext() {
		implements_selection_blocking_ext::<SelectionProxyBlocking<'static>>();
	}
}
