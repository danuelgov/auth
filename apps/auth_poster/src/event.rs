pub mod signup_requested;
pub mod user_created;
pub mod user_login_success;

pub use signup_requested::*;
pub use user_created::*;
pub use user_login_success::*;

use crate::EmailClient;

#[derive(Debug, Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum Event {
    #[serde(rename = "auth.signup.requested")]
    SignupRequest(SignupRequestedEvent),

    #[serde(rename = "auth.user.created")]
    UserCreated(UserCreatedEvent),

    #[serde(rename = "auth.user.login.success")]
    UserLoginSuccess(UserLoginSuccessEvent),
}

#[derive(Debug)]
pub enum EventError {
    SignupRequest(SignupRequestedEventError),
    UserCreated(UserCreatedEventError),
    UserLoginSuccess(UserLoginSuccessEventError),
}

pub async fn execute(event: Event, email_client: &EmailClient) -> Result<(), EventError> {
    macro_rules! execute {
            ($($variant:ident -> $error:ident,)+) => {
                match event {
                    $(
                        Event::$variant(event) => {
                            event
                                .execute(email_client)
                                .await
                                .map_err(EventError::$variant)?;
                        }
                    )+
                }

                Ok(())
            };
        }

    execute! {
        SignupRequest -> SignupRequestedEventError,
        UserCreated -> UserCreatedEventError,
        UserLoginSuccess -> UserLoginSuccessEventError,
    }
}
