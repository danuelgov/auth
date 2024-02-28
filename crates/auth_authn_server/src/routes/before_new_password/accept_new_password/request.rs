use auth_database::before_new_password::columns::BeforeNewPasswordIdentity;
use new_type::Password;

#[derive(Deserialize)]
pub struct Data {
    pub before_new_password_id: BeforeNewPasswordIdentity,
    pub password: Password,
}
