use crate::table::{Table, TableBlocking, TableProxy, TableProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait TableExtError: crate::table::Table {
	type Error: std::error::Error;
}
pub trait TableBlockingExtError: crate::table::TableBlocking {
	type Error: std::error::Error;
}

pub trait TableExt {}
pub trait TableBlockingExt {}

impl<T: TableExtError + crate::table::Table> TableExt for T {}
impl<T: TableBlockingExtError + crate::table::TableBlocking> TableBlockingExt for T {}

assert_impl_all!(TableProxy: Table, TableExt);
assert_impl_all!(TableProxyBlocking: TableBlocking, TableBlockingExt);
