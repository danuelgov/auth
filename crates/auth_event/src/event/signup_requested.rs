use crate::Event;

#[derive(Debug)]
pub struct SignupRequestedEvent(Event);

impl AsRef<Event> for SignupRequestedEvent {
    fn as_ref(&self) -> &Event {
        &self.0
    }
}

impl SignupRequestedEvent {
    pub fn new(email: String, before_signup_id: String) -> Result<Self, crate::EventError> {
        #[derive(Serialize)]
        struct Payload {
            email: String,
            before_signup_id: String,
        }

        let payload = Payload {
            email,
            before_signup_id,
        };
        let message = crate::Event::new(crate::AUTH_TOPIC, "auth.signup.requested", payload)?;

        Ok(Self(message))
    }
}
