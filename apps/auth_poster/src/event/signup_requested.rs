use crate::{EmailClient, EmailClientError, EmailPayload};

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
        println!(
            "User with email {} has requested signup",
            self.email_address
        );

        Ok(())
    }
}
