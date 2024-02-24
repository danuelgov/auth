use new_type::{EmailAddress, Password};

#[derive(Debug, Deserialize)]
pub struct Data {
    pub email_address: EmailAddress,
    pub password: Password,
}
