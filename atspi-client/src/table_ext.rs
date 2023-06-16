use atspi_proxies::table::{Table, TableBlocking, TableProxy, TableProxyBlocking};

impl_extended_errors!(TableProxy<'_>, TableExtError);
impl_extended_errors!(TableProxyBlocking<'_>, TableBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait TableExtError: Table {
	type Error: std::error::Error;
}
pub trait TableBlockingExtError: TableBlocking {
	type Error: std::error::Error;
}

pub trait TableExt {}
pub trait TableBlockingExt {}

impl<T: TableExtError + Table> TableExt for T {}
impl<T: TableBlockingExtError + TableBlocking> TableBlockingExt for T {}

assert_impl_all!(TableProxy: Table, TableExt);
assert_impl_all!(TableProxyBlocking: TableBlocking, TableBlockingExt);
