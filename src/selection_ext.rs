pub trait SelectionExtError: crate::selection::Selection {
	type Error: std::error::Error;
}

pub trait SelectionExt {
}

impl<T: SelectionExtError + crate::selection::Selection> SelectionExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	selection_ext::SelectionExt,
	selection::{SelectionProxy,
	SelectionProxyBlocking}};	fn implements_selection_ext<T: SelectionExt>() {}
	#[test]
	fn check_selection_implements_selection_ext() {
		implements_selection_ext::<SelectionProxy<'static>>();
	}
	#[test]
	fn check_blocking_selection_implements_selection_ext() {
		implements_selection_ext::<SelectionProxyBlocking<'static>>();
	}
}