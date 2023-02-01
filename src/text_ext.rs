use async_trait::async_trait;

#[allow(clippy::module_name_repetitions)]
pub trait TextExtError: crate::text::Text {
	type Error: std::error::Error
		+ From<<Self as crate::text::Text>::Error>
		+ Send
		+ Sync;
}

#[allow(clippy::module_name_repetitions)]
pub trait TextBlockingExtError: crate::text::TextBlocking {
	type Error: std::error::Error
		+ From<<Self as crate::text::TextBlocking>::Error>;
}

#[async_trait]
pub trait TextExt: TextExtError {
    async fn get_all_text(&self) -> Result<String, <Self as TextExtError>::Error>;
}

pub trait TextBlockingExt: TextBlockingExtError {
    fn get_all_text(&self) -> Result<String, <Self as TextBlockingExtError>::Error>;
}

#[async_trait]
impl<T: crate::text::Text + TextExtError + Send + Sync> TextExt for T {
    async fn get_all_text(&self) -> Result<String, <T as TextExtError>::Error> {
        let length_of_string = self.character_count().await?;
        Ok(self.get_text(0, length_of_string).await?)
    }
}

impl<T: crate::text::TextBlocking + TextBlockingExtError> TextBlockingExt for T {
    fn get_all_text(&self) -> Result<String, <T as TextBlockingExtError>::Error> {
        let length_of_string = self.character_count()?;
        Ok(self.get_text(0, length_of_string)?)
    }
}

#[cfg(test)]
mod tests {
	use crate::{
		text::TextProxy,
		text_ext::TextExt,
	};

	fn implements_text_ext<T: TextExt>() {}
	fn implements_text_blocking_ext<T: TextBlockingExt>() {}
	#[test]
	fn test_text_proxy_implement_text_ext() {
		implements_text_ext::<TextProxy<'static>>();
	}
	#[test]
	fn test_blocking_text_proxy_implement_text_ext() {
		implements_text_blocking_ext::<TextProxyBlocki9ng<'static>>();
	}
}
