use auth_database::before_signup::columns::BeforeSignupIdentity;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub before_signup_id: BeforeSignupIdentity,
}
