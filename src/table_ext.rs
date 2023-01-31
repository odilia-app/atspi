pub trait TableExtError: crate::table::Table {
	type Error: std::error::Error;
}

pub trait TableExt {
}

impl<T: TableExtError + crate::table::Table> TableExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	table_ext::TableExt,
	table::{TableProxy,
	TableProxyBlocking}};	fn implements_table_ext<T: TableExt>() {}
	#[test]
	fn check_table_implements_table_ext() {
		implements_table_ext::<TableProxy<'static>>();
	}
	#[test]
	fn check_blocking_table_implements_table_ext() {
		implements_table_ext::<TableProxyBlocking<'static>>();
	}
}