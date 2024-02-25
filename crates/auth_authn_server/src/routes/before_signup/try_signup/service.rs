use super::{
    create_before_signup::CreateBeforeSignupError, exists_email_address::ExistsEmailAddressError,
    exists_user_name::ExistsUserNameError, send_email::SendEmailError, RepositoryContract,
};
use auth_database::{
    agreement::columns::AgreementIdentity, hasher::columns::HasherPrimaryKey,
    user_profile::columns::UserProfileName,
};
use database_toolkit::DatabaseConnectionPool;
use new_type::{EmailAddress, Hasher, Password};

pub trait ServiceContract {
    async fn execute(&self) -> Result<(), ServiceError>;
}

pub struct Service<Repository: RepositoryContract> {
    pub pool: DatabaseConnectionPool,
    pub repository: Repository,
    pub email_address: EmailAddress,
    pub password: Password,
    pub name: UserProfileName,
    pub agreements: Vec<AgreementIdentity>,
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseConnectionPool(sqlx::Error),
    EmailAddressAlreadyExists,
    NameAlreadyExists,
    InvalidAgreement,
    PasswordHash,
    CreateBeforeSignup(CreateBeforeSignupError),
    SendEmail(SendEmailError),
}

impl<Repository: RepositoryContract> ServiceContract for Service<Repository> {
    async fn execute(&self) -> Result<(), ServiceError> {
        let email_address_handle = {
            let connection = self
                .pool
                .connection()
                .await
                .map_err(ServiceError::DatabaseConnectionPool)?;
            self.repository
                .exists_email_address(connection, self.email_address.clone())
        };

        let name_handle = {
            let connection = self
                .pool
                .connection()
                .await
                .map_err(ServiceError::DatabaseConnectionPool)?;
            self.repository
                .exists_user_name(connection, self.name.clone())
        };

        let (email_address_exists, name_exists) = tokio::join!(email_address_handle, name_handle);
        if email_address_exists? {
            return Err(ServiceError::EmailAddressAlreadyExists);
        }
        if name_exists? {
            return Err(ServiceError::NameAlreadyExists);
        }

        let agreements: Vec<_> = self
            .agreements
            .iter()
            .map(|agreement| agreement.primary_key())
            .collect();
        if agreements.len() != self.agreements.len() {
            return Err(ServiceError::InvalidAgreement);
        }

        let before_signup_id = {
            let hasher = Hasher::Argon2;
            let hasher_pk = HasherPrimaryKey::ARGON2;
            let hash = self
                .password
                .hash(hasher)
                .await
                .map_err(|_| ServiceError::PasswordHash)?;
            let connection = self
                .pool
                .connection()
                .await
                .map_err(ServiceError::DatabaseConnectionPool)?;
            self.repository
                .create_before_signup(
                    connection,
                    self.email_address.clone(),
                    hasher_pk,
                    hash,
                    self.name.clone(),
                    agreements,
                )
                .await
                .map_err(ServiceError::CreateBeforeSignup)?
        };

        self.repository
            .send_email(self.email_address.clone(), before_signup_id)
            .await
            .map_err(ServiceError::SendEmail)?;

        Ok(())
    }
}

impl From<ExistsEmailAddressError> for ServiceError {
    fn from(_: ExistsEmailAddressError) -> Self {
        ServiceError::EmailAddressAlreadyExists
    }
}

impl From<ExistsUserNameError> for ServiceError {
    fn from(_: ExistsUserNameError) -> Self {
        ServiceError::NameAlreadyExists
    }
}
