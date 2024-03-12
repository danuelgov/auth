use aws_sdk_ses::{
    types::{Body, Content, Destination, Message},
    Client,
};

#[derive(Debug, Clone)]
pub struct EmailClient {
    inner: Client,
}

#[derive(Debug)]
pub enum EmailClientError {
    Aws(aws_sdk_ses::Error),
    Subject(aws_sdk_ses::error::BuildError),
    HtmlBody(aws_sdk_ses::error::BuildError),
    TextBody(aws_sdk_ses::error::BuildError),
}

pub struct EmailPayload {
    pub to_addresses: Vec<String>,
    pub from_address: &'static str,
    pub cc_addresses: Vec<String>,
    pub bcc_addresses: Vec<String>,
    pub subject: String,
    pub body_html: String,
    pub body_text: String,
}

impl EmailClient {
    pub async fn new() -> Self {
        let inner = aws_config::from_env().load().await;
        let inner = Client::new(&inner);

        Self { inner }
    }

    pub async fn send(&self, payload: EmailPayload) -> Result<(), EmailClientError> {
        self.inner
            .send_email()
            .destination(payload.destination())
            .message(payload.message()?)
            .send()
            .await
            .map_err(|error| EmailClientError::Aws(error.into()))?;

        Ok(())
    }

    pub async fn send_template(
        &self,
        template_name: &str,
        payload: EmailPayload,
    ) -> Result<(), EmailClientError> {
        self.inner
            .send_templated_email()
            .destination(payload.destination())
            .template(template_name)
            .template_data(&payload.body_html)
            .send()
            .await
            .map_err(|error| EmailClientError::Aws(error.into()))?;

        Ok(())
    }
}

impl EmailPayload {
    fn destination(&self) -> Destination {
        let mut destination = Destination::builder();
        for address in &self.cc_addresses {
            destination = destination.cc_addresses(address);
        }
        for address in &self.bcc_addresses {
            destination = destination.bcc_addresses(address);
        }
        for address in &self.to_addresses {
            destination = destination.to_addresses(address);
        }
        destination.build()
    }

    fn message(&self) -> Result<Message, EmailClientError> {
        macro_rules! content {
            ($error:ident($source:ident)) => {
                Content::builder()
                    .data(&self.$source)
                    .build()
                    .map_err(EmailClientError::$error)
            };
        }

        let subject = content!(Subject(subject))?;
        let html = content!(HtmlBody(body_html))?;
        let text = content!(TextBody(body_text))?;
        let body = Body::builder().html(html).text(text).build();
        let message = Message::builder().subject(subject).body(body).build();

        Ok(message)
    }
}
