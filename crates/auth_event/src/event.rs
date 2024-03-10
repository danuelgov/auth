pub mod signup_requested;
pub mod user_created;

pub use signup_requested::*;
pub use user_created::*;

use crate::Topic;
use serde::Serialize;

#[derive(Debug)]
pub struct Event {
    topic: Topic,
    message: String,
}

#[derive(Debug)]
pub enum EventError {
    Serialization(serde_json::Error),
}

impl Event {
    pub fn new<P>(topic: Topic, action: &'static str, payload: P) -> Result<Self, EventError>
    where
        P: Serialize,
    {
        #[derive(Serialize)]
        pub struct Message<P>
        where
            P: Serialize,
        {
            action: &'static str,
            payload: P,
        }

        let message = serde_json::to_string(&Message { action, payload })
            .map_err(EventError::Serialization)?;

        Ok(Self { topic, message })
    }

    #[inline]
    pub const fn topic_arn(&self) -> &str {
        self.topic.arn()
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }
}
