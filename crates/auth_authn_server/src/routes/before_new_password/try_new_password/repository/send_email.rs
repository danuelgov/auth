use auth_database::before_new_password::columns::BeforeNewPasswordIdentity;
use new_type::EmailAddress;

pub trait SendEmailContract {
    async fn send_email(
        &self,
        email_address: EmailAddress,
        before_new_password_id: BeforeNewPasswordIdentity,
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
        before_new_password_id: BeforeNewPasswordIdentity,
    ) -> Result<(), SendEmailError> {
        dbg!(email_address, before_new_password_id);

        // TODO: 이메일 전송 연동
        // Tracking Issue: https://github.com/danuelgov/auth/issues/4

        Ok(())
    }
}
