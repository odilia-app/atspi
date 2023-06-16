use async_trait::async_trait;
use atspi_proxies::text::{Text, TextBlocking, TextProxy, TextProxyBlocking};

impl_extended_errors!(TextProxy<'_>, TextExtError);
impl_extended_errors!(TextProxyBlocking<'_>, TextBlockingExtError);

#[allow(clippy::module_name_repetitions)]
pub trait TextExtError: Text {
	type Error: std::error::Error + From<<Self as Text>::Error> + Send + Sync;
}

#[allow(clippy::module_name_repetitions)]
pub trait TextBlockingExtError: TextBlocking {
	type Error: std::error::Error + From<<Self as TextBlocking>::Error>;
}

#[async_trait]
pub trait TextExt: TextExtError {
	/// Gets the full text within the accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation of [`Text::get_text`] or [`TextBlocking::get_text`].
	/// With the [`TextProxy`] and [`TextProxyBlocking`] implmentations, this can fail if you ask for an invalid start or end index, or if the `DBus` method fails to send or receive.
	async fn get_all_text(&self) -> Result<String, <Self as TextExtError>::Error>;
}

pub trait TextBlockingExt: TextBlockingExtError {
	/// Gets the full text within the accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation of [`Text::get_text`] or [`TextBlocking::get_text`].
	/// With the [`TextProxy`] and [`TextProxyBlocking`] implmentations, this can fail if you ask for an invalid start or end index, or if the `DBus` method fails to send or receive.
	fn get_all_text(&self) -> Result<String, <Self as TextBlockingExtError>::Error>;
}

#[async_trait]
impl<T: Text + TextExtError + Send + Sync> TextExt for T {
	async fn get_all_text(&self) -> Result<String, <T as TextExtError>::Error> {
		let length_of_string = self.character_count().await?;
		Ok(self.get_text(0, length_of_string).await?)
	}
}

impl<T: TextBlocking + TextBlockingExtError> TextBlockingExt for T {
	fn get_all_text(&self) -> Result<String, <T as TextBlockingExtError>::Error> {
		let length_of_string = self.character_count()?;
		Ok(self.get_text(0, length_of_string)?)
	}
}

assert_impl_all!(TextProxy: Text, TextExt);
assert_impl_all!(TextProxyBlocking: TextBlocking, TextBlockingExt);
