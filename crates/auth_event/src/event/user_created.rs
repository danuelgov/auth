use crate::{Event, AUTH_TOPIC};

#[derive(Debug)]
pub struct UserCreatedEvent(Event);

impl AsRef<Event> for UserCreatedEvent {
    fn as_ref(&self) -> &Event {
        &self.0
    }
}

impl UserCreatedEvent {
    pub fn new(user_id: String, email_address: String) -> Result<Self, crate::EventError> {
        #[derive(Serialize)]
        struct Payload {
            user_id: String,
            email_address: String,
        }

        let payload = Payload {
            user_id,
            email_address,
        };
        let message = crate::Event::new(AUTH_TOPIC, "auth.user.created", payload)?;

        Ok(Self(message))
    }
}
