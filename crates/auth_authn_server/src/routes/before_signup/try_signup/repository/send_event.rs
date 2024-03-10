use auth_database::before_signup::columns::BeforeSignupIdentity;
use auth_event::{EventClient, EventClientError, EventError, SignupRequestedEvent};
use new_type::EmailAddress;

pub trait SendEventContract {
    async fn send_event(
        &self,
        client: EventClient,
        email_address: EmailAddress,
        before_signup_id: BeforeSignupIdentity,
    ) -> Result<(), SendEventError>;
}

#[derive(Debug)]
pub enum SendEventError {
    Event(EventError),
    Send(EventClientError),
}

impl SendEventContract for super::Repository {
    async fn send_event(
        &self,
        client: EventClient,
        email_address: EmailAddress,
        before_signup_id: BeforeSignupIdentity,
    ) -> Result<(), SendEventError> {
        let email_address = email_address.to_string();
        let before_signup_id = before_signup_id.to_string();
        client
            .send(
                SignupRequestedEvent::new(email_address, before_signup_id)
                    .map_err(SendEventError::Event)?,
            )
            .await
            .map_err(SendEventError::Send)?;

        Ok(())
    }
}
