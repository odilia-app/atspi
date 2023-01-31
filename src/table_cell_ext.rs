pub trait TableCellExtError: crate::table_cell::TableCell {
	type Error: std::error::Error;
}

pub trait TableCellExt {
}

impl<T: TableCellExtError + crate::table_cell::TableCell> TableCellExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	table_cell_ext::TableCellExt,
	table_cell::{TableCellProxy,
	TableCellProxyBlocking}};	fn implements_table_cell_ext<T: TableCellExt>() {}
	#[test]
	fn check_table_cell_implements_table_cell_ext() {
		implements_table_cell_ext::<TableCellProxy<'static>>();
	}
	#[test]
	fn check_blocking_table_cell_implements_table_cell_ext() {
		implements_table_cell_ext::<TableCellProxyBlocking<'static>>();
	}
}