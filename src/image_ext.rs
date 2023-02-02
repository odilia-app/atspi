pub trait ImageExtError: crate::image::Image {
	type Error: std::error::Error;
}
pub trait ImageBlockingExtError: crate::image::ImageBlocking {
	type Error: std::error::Error;
}

pub trait ImageExt {
}
pub trait ImageBlockingExt {
}

impl<T: ImageExtError + crate::image::Image> ImageExt for T {
}
impl<T: ImageBlockingExtError + crate::image::ImageBlocking> ImageBlockingExt for T {
}

#[cfg(test)]
mod test {
  use crate::{
    image_ext::{
      ImageExt,
      ImageBlockingExt,
    },
    image::{
      ImageProxy,
      ImageProxyBlocking
    },
  };
  fn implements_image_ext<T: ImageExt>() {}
  fn implements_image_blocking_ext<T: ImageBlockingExt>() {}
	#[test]
	fn check_image_implements_image_ext() {
		implements_image_ext::<ImageProxy<'static>>();
	}
	#[test]
	fn check_blocking_image_implements_image_ext() {
		implements_image_blocking_ext::<ImageProxyBlocking<'static>>();
	}
}
