pub trait ImageExtError: crate::image::Image {
	type Error: std::error::Error;
}

pub trait ImageExt {
}

impl<T: ImageExtError + crate::image::Image> ImageExt for T {
}

#[cfg(test)]
mod test {
use crate::{
	image_ext::ImageExt,
	image::{ImageProxy,
	ImageProxyBlocking}};	fn implements_image_ext<T: ImageExt>() {}
	#[test]
	fn check_image_implements_image_ext() {
		implements_image_ext::<ImageProxy<'static>>();
	}
	#[test]
	fn check_blocking_image_implements_image_ext() {
		implements_image_ext::<ImageProxyBlocking<'static>>();
	}
}