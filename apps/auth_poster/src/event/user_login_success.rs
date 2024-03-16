mod email_template;

use crate::{EmailClient, EmailClientError, EmailPayload};
use email_template::*;

#[derive(Debug, Deserialize)]
pub struct UserLoginSuccessEvent {
    pub email_address: String,
    pub user_id: String,
}

#[derive(Debug)]
pub enum UserLoginSuccessEventError {
    Email(EmailClientError),
}

impl UserLoginSuccessEvent {
    pub async fn execute(
        &self,
        email_client: &EmailClient,
    ) -> Result<(), UserLoginSuccessEventError> {
        email_client
            .send(EmailPayload {
                from_address: "noreply@danuel.io",
                to_addresses: vec![self.email_address.clone()],
                cc_addresses: vec![],
                bcc_addresses: vec![],
                subject: "[다뉴엘 거버넌스] 새로운 로그인".to_owned(),
                body: EmailTemplate {
                    email_address: self.email_address.to_owned(),
                }
                .render(),
            })
            .await
            .map_err(UserLoginSuccessEventError::Email)?;

        Ok(())
    }
}
