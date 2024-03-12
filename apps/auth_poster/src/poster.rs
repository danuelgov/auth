use aws_sdk_sqs::{types::Message, Client};

pub struct PosterClient {
    url: &'static str,
    inner: Client,
}

#[derive(Debug)]
pub enum PosterClientError {
    Aws(aws_sdk_sqs::Error),
}

impl PosterClient {
    pub async fn new(url: &'static str) -> Self {
        let inner = aws_config::from_env().load().await;
        let inner = Client::new(&inner);

        Self { url, inner }
    }

    pub async fn receive(&self) -> Result<Vec<Message>, PosterClientError> {
        let received = self
            .inner
            .receive_message()
            .queue_url(self.url)
            .send()
            .await
            .map_err(|error| PosterClientError::Aws(error.into()))?;

        Ok(received.messages.unwrap_or_default())
    }

    pub async fn commit(&self, receipt_handle: &str) -> Result<(), PosterClientError> {
        self.inner
            .delete_message()
            .queue_url(self.url)
            .receipt_handle(receipt_handle)
            .send()
            .await
            .map_err(|error| PosterClientError::Aws(error.into()))?;

        Ok(())
    }
}
