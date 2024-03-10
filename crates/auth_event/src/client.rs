use crate::Event;

#[derive(Debug, Clone)]
pub struct EventClient {
    inner: aws_sdk_sns::Client,
}

#[derive(Debug)]
pub enum EventClientError {
    Aws(aws_sdk_sns::Error),
}

impl EventClient {
    #[inline]
    pub async fn new() -> Self {
        let inner = aws_config::from_env().load().await;
        let inner = aws_sdk_sns::Client::new(&inner);

        Self { inner }
    }

    pub async fn send<E>(&self, event: E) -> Result<(), EventClientError>
    where
        E: AsRef<Event>,
    {
        let event = event.as_ref();
        self.inner
            .publish()
            .topic_arn(event.topic_arn())
            .message(event.message())
            .send()
            .await
            .map_err(|error| EventClientError::Aws(error.into()))?;

        Ok(())
    }
}
