macro_rules! impl_extended_errors {
	($proxy:ty, $error:ty) => {
		impl $error for $proxy {
			type Error = atspi_common::error::AtspiError;
		}
	};
}
