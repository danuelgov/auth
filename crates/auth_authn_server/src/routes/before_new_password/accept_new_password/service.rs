use super::{
    create_password::CreatePasswordError, expire_before_new_password::ExpireBeforeNewPasswordError,
    expire_previous_password::ExpirePreviousPasswordError,
    find_before_new_password::FindBeforeNewPasswordError, send_email::SendEmailError,
    RepositoryContract,
};
use crate::routes::before_new_password::accept_new_password::find_before_new_password::FindBeforeNewPasswordEntity;
use auth_database::{
    before_new_password::columns::BeforeNewPasswordIdentity, hasher::columns::HasherPrimaryKey,
    user_credential__has__hasher::columns::UserCredentialHasHasherPrimaryKey,
};
use database_toolkit::DatabaseConnectionPool;
use new_type::{Hasher, HasherError, Password};

pub trait ServiceContract {
    async fn execute(&self) -> Result<(), ServiceError>;
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseConnectionPool(sqlx::Error),
    Transaction(sqlx::Error),
    FindBeforeNewPassword(FindBeforeNewPasswordError),
    ExpireBeforeNewPassword(ExpireBeforeNewPasswordError),
    ExpirePreviousPassword(ExpirePreviousPasswordError),
    PasswordHash(HasherError),
    CreatePassword(CreatePasswordError),
    SendEmail(SendEmailError),
}

pub struct Service<Repository: RepositoryContract> {
    pub pool: DatabaseConnectionPool,
    pub repository: Repository,
    pub before_new_password_id: BeforeNewPasswordIdentity,
    pub password: Password,
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

        let FindBeforeNewPasswordEntity {
            before_new_password_pk,
            user_credential_pk,
            email_address,
        } = {
            let now = chrono::Utc::now();
            self.repository
                .find_before_new_password(connection!(), self.before_new_password_id, now)
                .await
                .map_err(ServiceError::FindBeforeNewPassword)?
        };
        let mut transaction = transaction!();
        {
            let now = chrono::Utc::now();
            self.repository
                .expire_before_new_password(&mut transaction, before_new_password_pk, now)
                .await
                .map_err(ServiceError::ExpireBeforeNewPassword)?;
        }
        {
            let now = chrono::Utc::now();
            self.repository
                .expire_previous_password(&mut transaction, user_credential_pk, now)
                .await
                .map_err(ServiceError::ExpirePreviousPassword)?;
        }
        {
            let user_credential__has__hasher_pk = UserCredentialHasHasherPrimaryKey::new();
            let hasher_pk = HasherPrimaryKey::new();
            let hasher = Hasher::Argon2;
            let hash = self
                .password
                .hash(hasher)
                .await
                .map_err(ServiceError::PasswordHash)?;
            let expired_at = chrono::Utc::now() + chrono::Duration::days(90);
            self.repository
                .create_password(
                    &mut transaction,
                    user_credential__has__hasher_pk,
                    user_credential_pk,
                    hasher_pk,
                    hash,
                    expired_at,
                )
                .await
                .map_err(ServiceError::CreatePassword)?;
        }
        {
            self.repository
                .send_email(email_address)
                .await
                .map_err(ServiceError::SendEmail)?;
        }

        transaction
            .commit()
            .await
            .map_err(ServiceError::Transaction)?;

        Ok(())
    }
}
