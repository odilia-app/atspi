use async_trait::async_trait;

pub trait TextExtError: crate::text::Text {
	type Error: std::error::Error
		+ From<<Self as crate::text::Text>::Error>
		+ Send
		+ Sync;
}

#[async_trait]
pub trait TextExt: TextExtError {
    async fn get_all_text(&self) -> Result<String, <Self as TextExtError>::Error>;
}

#[async_trait]
impl<T: crate::text::Text + TextExtError + Send + Sync> TextExt for T {
    async fn get_all_text(&self) -> Result<String, <T as TextExtError>::Error> {
        let length_of_string = self.character_count().await?;
        Ok(self.get_text(0, length_of_string).await?)
    }
}

#[cfg(test)]
mod tests {
	use crate::{
		text::TextProxy,
		text_ext::TextExt,
	};

	fn implements_text_ext<T: TextExt>() {}
	#[test]
	fn test_text_proxy_implement_text_ext() {
		implements_text_ext::<TextProxy<'static>>();
	}
}
