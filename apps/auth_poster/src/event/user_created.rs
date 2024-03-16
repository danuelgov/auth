mod email_template;

use crate::{EmailClient, EmailClientError, EmailPayload};
use email_template::*;

#[derive(Debug, Deserialize)]
pub struct UserCreatedEvent {
    pub email_address: String,
    pub user_id: String,
}

#[derive(Debug)]
pub enum UserCreatedEventError {
    Email(EmailClientError),
}

impl UserCreatedEvent {
    pub async fn execute(&self, email_client: &EmailClient) -> Result<(), UserCreatedEventError> {
        email_client
            .send(EmailPayload {
                from_address: "noreply@danuel.io",
                to_addresses: vec![self.email_address.clone()],
                cc_addresses: vec![],
                bcc_addresses: vec![],
                subject: "[다뉴엘 거버넌스] 계정 생성 완료".to_owned(),
                body: EmailTemplate {
                    email_address: self.email_address.to_owned(),
                }
                .render(),
            })
            .await
            .map_err(UserCreatedEventError::Email)?;

        Ok(())
    }
}
