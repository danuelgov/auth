use crate::EmailClient;

#[derive(Debug, Deserialize)]
pub struct UserLoginSuccessEvent {
    pub email_address: String,
    pub user_id: String,
}

#[derive(Debug)]
pub enum UserLoginSuccessEventError {
    //
}

impl UserLoginSuccessEvent {
    pub async fn execute(
        &self,
        email_client: &EmailClient,
    ) -> Result<(), UserLoginSuccessEventError> {
        println!("User login success: {:?}", self);

        Ok(())
    }
}
