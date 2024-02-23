use auth_database::before_signup::columns::BeforeSignupIdentity;
use new_type::EmailAddress;

pub trait SendEmailContract {
    async fn send_email(
        &self,
        email_address: EmailAddress,
        before_signup_id: BeforeSignupIdentity,
    ) -> Result<(), SendEmailError>;
}

#[derive(Debug)]
pub enum SendEmailError {
    //
}

impl SendEmailContract for super::Repository {
    async fn send_email(
        &self,
        email_address: EmailAddress,
        before_signup_id: BeforeSignupIdentity,
    ) -> Result<(), SendEmailError> {
        dbg!(email_address, before_signup_id);

        // TODO: 이메일 전송 연동

        Ok(())
    }
}
