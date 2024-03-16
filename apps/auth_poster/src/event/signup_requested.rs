mod email_template;

use crate::{EmailClient, EmailClientError, EmailPayload};
use email_template::*;

#[derive(Debug, Deserialize)]
pub struct SignupRequestedEvent {
    pub email_address: String,
    pub verification_code: String,
}

#[derive(Debug)]
pub enum SignupRequestedEventError {
    Email(EmailClientError),
}

impl SignupRequestedEvent {
    pub async fn execute(
        &self,
        email_client: &EmailClient,
    ) -> Result<(), SignupRequestedEventError> {
        email_client
            .send(EmailPayload {
                from_address: "noreply@danuel.io",
                to_addresses: vec![self.email_address.clone()],
                cc_addresses: vec![],
                bcc_addresses: vec![],
                subject: "[다뉴엘 거버넌스] 회원가입 인증".to_owned(),
                body: EmailTemplate {
                    email_address: self.email_address.to_owned(),
                    // TODO: 회원가입 프론트엔드 서버 주소로 변경
                    verification_url: format!(
                        "http://localhost:8080/signup/verify?verification_code={}",
                        self.verification_code
                    ),
                }
                .render(),
            })
            .await
            .map_err(SignupRequestedEventError::Email)?;

        Ok(())
    }
}
