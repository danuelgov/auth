use auth_database::user::columns::UserPrimaryKey;
use auth_event::{EventClient, EventClientError, EventError, UserLoginSuccessEvent};
use new_type::EmailAddress;

pub trait SendEventContract {
    async fn send_event(
        &self,
        client: EventClient,
        user_pk: UserPrimaryKey,
        email_address: EmailAddress,
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
        user_pk: UserPrimaryKey,
        email_address: EmailAddress,
    ) -> Result<(), SendEventError> {
        let email_address = email_address.to_string();
        let user_pk = user_pk.to_string();
        client
            .send(
                UserLoginSuccessEvent::new(email_address, user_pk)
                    .map_err(SendEventError::Event)?,
            )
            .await
            .map_err(SendEventError::Send)?;

        Ok(())
    }
}
