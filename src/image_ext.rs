use crate::image::{Image, ImageBlocking, ImageProxy, ImageProxyBlocking};

#[allow(clippy::module_name_repetitions)]
pub trait ImageExtError: crate::image::Image {
	type Error: std::error::Error;
}
pub trait ImageBlockingExtError: crate::image::ImageBlocking {
	type Error: std::error::Error;
}

pub trait ImageExt {}
pub trait ImageBlockingExt {}

impl<T: ImageExtError + crate::image::Image> ImageExt for T {}
impl<T: ImageBlockingExtError + crate::image::ImageBlocking> ImageBlockingExt for T {}

assert_impl_all!(ImageProxy: Image, ImageExt);
assert_impl_all!(ImageProxyBlocking: ImageBlocking, ImageBlockingExt);
