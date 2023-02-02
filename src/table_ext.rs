pub trait TableExtError: crate::table::Table {
	type Error: std::error::Error;
}
pub trait TableBlockingExtError: crate::table::TableBlocking {
	type Error: std::error::Error;
}

pub trait TableExt {
}
pub trait TableBlockingExt {
}

impl<T: TableExtError + crate::table::Table> TableExt for T {
}
impl<T: TableBlockingExtError + crate::table::TableBlocking> TableBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    table_ext::{
      TableExt,
      TableBlockingExt,
    },
    table::{
      TableProxy,
      TableProxyBlocking
    }
  };
  fn implements_table_ext<T: TableExt>() {}
  fn implements_table_blocking_ext<T: TableBlockingExt>() {}
	#[test]
	fn check_table_implements_table_ext() {
		implements_table_ext::<TableProxy<'static>>();
	}
	#[test]
	fn check_blocking_table_implements_table_ext() {
		implements_table_blocking_ext::<TableProxyBlocking<'static>>();
	}
}
