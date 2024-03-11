use auth_database::{
    agreement::columns::AgreementIdentity, user_profile::columns::UserProfileName,
};
use new_type::{email_address::EmailAddress, password::Password};

#[derive(Debug, Deserialize)]
pub struct Data {
    pub email_address: EmailAddress,
    pub password: Password,
    pub name: UserProfileName,
    pub agreements: Vec<AgreementIdentity>,
}
