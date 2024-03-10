use crate::{Event, AUTH_TOPIC};

#[derive(Debug)]
pub struct UserLoginSuccessEvent(Event);

impl AsRef<Event> for UserLoginSuccessEvent {
    fn as_ref(&self) -> &Event {
        &self.0
    }
}

impl UserLoginSuccessEvent {
    pub fn new(user_pk: String, email_address: String) -> Result<Self, crate::EventError> {
        #[derive(Serialize)]
        struct Payload {
            user_pk: String,
            email_address: String,
        }

        let payload = Payload {
            user_pk,
            email_address,
        };
        let message = crate::Event::new(AUTH_TOPIC, "auth.user.login.success", payload)?;

        Ok(Self(message))
    }
}
