use crate::Event;

#[derive(Debug, Clone)]
pub struct Client {
    inner: aws_sdk_sns::Client,
}

impl Client {
    #[inline]
    pub async fn new<S>() -> Self
    where
        S: Into<String>,
    {
        let inner = aws_config::from_env().load().await;
        let inner = aws_sdk_sns::Client::new(&inner);

        Self { inner }
    }

    pub async fn send<E>(&self, event: E) -> Result<(), aws_sdk_sns::Error>
    where
        E: AsRef<Event>,
    {
        let event = event.as_ref();
        self.inner
            .publish()
            .topic_arn(event.topic_arn())
            .message(event.message())
            .send()
            .await?;

        Ok(())
    }
}
