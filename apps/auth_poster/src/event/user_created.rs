use crate::EmailClient;

#[derive(Debug, Deserialize)]
pub struct UserCreatedEvent {
    pub email_address: String,
    pub user_id: String,
}

#[derive(Debug)]
pub enum UserCreatedEventError {
    //
}

impl UserCreatedEvent {
    pub async fn execute(&self, email_client: &EmailClient) -> Result<(), UserCreatedEventError> {
        println!("User created: {:?}", self);

        Ok(())
    }
}
