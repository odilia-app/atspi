pub trait TableCellExtError: crate::table_cell::TableCell {
	type Error: std::error::Error;
}
pub trait TableCellBlockingExtError: crate::table_cell::TableCellBlocking {
	type Error: std::error::Error;
}

pub trait TableCellExt {
}
pub trait TableCellBlockingExt {
}

impl<T: TableCellExtError + crate::table_cell::TableCell> TableCellExt for T {
}
impl<T: TableCellBlockingExtError + crate::table_cell::TableCellBlocking> TableCellBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    table_cell_ext::{
      TableCellExt,
      TableCellBlockingExt,
    },
    table_cell::{
      TableCellProxy,
      TableCellProxyBlocking,
    },
  };
  fn implements_table_cell_ext<T: TableCellExt>() {}
  fn implements_table_cell_blocking_ext<T: TableCellBlockingExt>() {}
	#[test]
	fn check_table_cell_implements_table_cell_ext() {
		implements_table_cell_ext::<TableCellProxy<'static>>();
	}
	#[test]
	fn check_blocking_table_cell_implements_table_cell_ext() {
		implements_table_cell_blocking_ext::<TableCellProxyBlocking<'static>>();
	}
}
