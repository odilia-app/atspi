use crate::table_cell::{TableCell, TableCellBlocking, TableCellProxy, TableCellProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait TableCellExtError: crate::table_cell::TableCell {
	type Error: std::error::Error;
}
pub trait TableCellBlockingExtError: crate::table_cell::TableCellBlocking {
	type Error: std::error::Error;
}

pub trait TableCellExt {}
pub trait TableCellBlockingExt {}

impl<T: TableCellExtError + crate::table_cell::TableCell> TableCellExt for T {}
impl<T: TableCellBlockingExtError + crate::table_cell::TableCellBlocking> TableCellBlockingExt
	for T
{
}

assert_impl_all!(TableCellProxy: TableCell, TableCellExt);
assert_impl_all!(TableCellProxyBlocking: TableCellBlocking, TableCellBlockingExt);
