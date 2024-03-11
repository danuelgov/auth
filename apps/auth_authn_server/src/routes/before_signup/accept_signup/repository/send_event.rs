use auth_database::user::columns::UserIdentity;
use auth_event::{EventClient, EventClientError, EventError, UserCreatedEvent};
use new_type::EmailAddress;

pub trait SendEventContract {
    async fn send_event(
        &self,
        client: EventClient,
        email_address: EmailAddress,
        user_id: UserIdentity,
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
        user_id: UserIdentity,
    ) -> Result<(), SendEventError> {
        let email_address = email_address.to_string();
        let user_id = user_id.to_string();
        client
            .send(UserCreatedEvent::new(email_address, user_id).map_err(SendEventError::Event)?)
            .await
            .map_err(SendEventError::Send)?;

        Ok(())
    }
}
