use atspi_proxies::image::{Image, ImageBlocking, ImageProxy, ImageProxyBlocking};

impl_extended_errors!(ImageProxy<'_>, ImageExtError);
impl_extended_errors!(ImageProxyBlocking<'_>, ImageBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait ImageExtError: Image {
	type Error: std::error::Error;
}
pub trait ImageBlockingExtError: ImageBlocking {
	type Error: std::error::Error;
}

pub trait ImageExt {}
pub trait ImageBlockingExt {}

impl<T: ImageExtError + Image> ImageExt for T {}
impl<T: ImageBlockingExtError + ImageBlocking> ImageBlockingExt for T {}

assert_impl_all!(ImageProxy: Image, ImageExt);
assert_impl_all!(ImageProxyBlocking: ImageBlocking, ImageBlockingExt);
