use super::{
    create_before_new_password::CreateBeforeNewPasswordError,
    expire_previous_before_password::ExpirePreviousBeforePasswordError,
    find_user_by_email_address::FindUserByEmailAddressError, send_email::SendEmailError,
    RepositoryContract,
};
use crate::routes::before_new_password::try_new_password::find_user_by_email_address::FindUserByEmailAddressEntity;
use auth_database::before_new_password::columns::{
    BeforeNewPasswordIdentity, BeforeNewPasswordPrimaryKey,
};
use database_toolkit::DatabaseConnectionPool;
use new_type::EmailAddress;

pub trait ServiceContract {
    async fn execute(&self) -> Result<(), ServiceError>;
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseConnectionPool(sqlx::Error),
    Transaction(sqlx::Error),
    FindUserByEmailAddress(FindUserByEmailAddressError),
    ExpirePreviousBeforePassword(ExpirePreviousBeforePasswordError),
    CreateBeforeNewPassword(CreateBeforeNewPasswordError),
    SendEmail(SendEmailError),
}

pub struct Service<Repository: RepositoryContract> {
    pub pool: DatabaseConnectionPool,
    pub repository: Repository,
    pub email_address: EmailAddress,
}

impl<Repository: RepositoryContract> ServiceContract for Service<Repository> {
    async fn execute(&self) -> Result<(), ServiceError> {
        macro_rules! connection {
            () => {{
                self.pool
                    .connection()
                    .await
                    .map_err(ServiceError::DatabaseConnectionPool)?
            }};
        }

        macro_rules! transaction {
            () => {{
                self.pool
                    .transaction()
                    .await
                    .map_err(ServiceError::Transaction)?
            }};
        }

        let FindUserByEmailAddressEntity { user_credential_pk } = self
            .repository
            .find_user_by_email_address(connection!(), self.email_address.to_owned())
            .await
            .map_err(ServiceError::FindUserByEmailAddress)?;
        let mut transaction = transaction!();
        {
            let now = chrono::Utc::now();
            self.repository
                .expire_previous_before_password(&mut transaction, user_credential_pk, now)
                .await
                .map_err(ServiceError::ExpirePreviousBeforePassword)?
        }
        let before_new_password_id = BeforeNewPasswordIdentity::new();
        {
            let before_new_password_pk = BeforeNewPasswordPrimaryKey::new();
            let expired_at = chrono::Utc::now() + chrono::Duration::days(1);
            self.repository
                .create_before_new_password(
                    &mut transaction,
                    before_new_password_pk,
                    before_new_password_id,
                    user_credential_pk,
                    expired_at,
                )
                .await
                .map_err(ServiceError::CreateBeforeNewPassword)?
        };
        {
            self.repository
                .send_email(self.email_address.to_owned(), before_new_password_id)
                .await
                .map_err(ServiceError::SendEmail)?
        }

        transaction
            .commit()
            .await
            .map_err(ServiceError::Transaction)?;

        Ok(())
    }
}
