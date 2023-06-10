use atspi_proxies::table_cell::{TableCell, TableCellBlocking, TableCellProxy, TableCellProxyBlocking};

impl_extended_errors!(TableCellProxy<'_>, TableCellExtError);
impl_extended_errors!(TableCellProxyBlocking<'_>, TableCellBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait TableCellExtError: TableCell {
	type Error: std::error::Error;
}
pub trait TableCellBlockingExtError: TableCellBlocking {
	type Error: std::error::Error;
}

pub trait TableCellExt {}
pub trait TableCellBlockingExt {}

impl<T: TableCellExtError + TableCell> TableCellExt for T {}
impl<T: TableCellBlockingExtError + TableCellBlocking> TableCellBlockingExt
	for T
{
}

assert_impl_all!(TableCellProxy: TableCell, TableCellExt);
assert_impl_all!(TableCellProxyBlocking: TableCellBlocking, TableCellBlockingExt);
