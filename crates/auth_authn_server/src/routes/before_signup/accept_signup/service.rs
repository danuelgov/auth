use super::{
    complete_before_signup::CompleteBeforeSignupError,
    create_user::CreateUserError,
    create_user_agreement::CreateUserAgreementError,
    create_user_credential::CreateUserCredentialError,
    create_user_credential_hasher::CreateUserCredentialHasherError,
    create_user_profile::CreateUserProfileError,
    find_before_signup::{BeforeSignupData, FindBeforeSignupError},
    RepositoryContract,
};
use auth_database::{
    before_signup::columns::BeforeSignupIdentity,
    user::columns::{UserIdentity, UserPrimaryKey},
    user_credential::columns::UserCredentialPrimaryKey,
    user_credential__has__hasher::columns::UserCredentialHasHasherPrimaryKey,
};
use database_toolkit::DatabaseConnectionPool;
use new_type::Handle;

pub trait ServiceContract {
    async fn execute(&self) -> Result<(), ServiceError>;
}

pub struct Service<Repository: RepositoryContract> {
    pub pool: DatabaseConnectionPool,
    pub repository: Repository,
    pub before_signup_id: BeforeSignupIdentity,
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseConnectionPool(sqlx::Error),
    FindBeforeSignup(FindBeforeSignupError),
    CreateUser(CreateUserError),
    CreateUserCredentialHasher(CreateUserCredentialHasherError),
    CreateUserCredential(CreateUserCredentialError),
    CreateUserProfile(CreateUserProfileError),
    CreateUserAgreement(CreateUserAgreementError),
    CompleteBeforeSignup(CompleteBeforeSignupError),
}

impl<Repository: RepositoryContract> ServiceContract for Service<Repository> {
    async fn execute(&self) -> Result<(), ServiceError> {
        macro_rules! invoke {
            ($name:ident($($arg:expr),+) else $variant:ident) => {{
                self.repository
                    .$name($($arg),*)
                    .await
                    .map_err(ServiceError::$variant)?
            }};
        }

        let BeforeSignupData {
            before_signup_pk,
            email_address,
            hasher_pk,
            hash,
            name,
            agreements,
        } = {
            let connection = self
                .pool
                .connection()
                .await
                .map_err(ServiceError::DatabaseConnectionPool)?;
            invoke!(find_before_signup(connection, self.before_signup_id) else FindBeforeSignup)
        };

        let mut transaction = self
            .pool
            .transaction()
            .await
            .map_err(ServiceError::DatabaseConnectionPool)?;
        let user_pk = UserPrimaryKey::new();
        let user_id = UserIdentity::new();
        let user_credential_pk = UserCredentialPrimaryKey::new();
        invoke!(create_user(&mut transaction, user_pk, user_id) else CreateUser);
        invoke!(create_user_credential(&mut transaction, user_credential_pk, user_pk, email_address) else CreateUserCredential);

        let user_credential__has__hasher_pk = UserCredentialHasHasherPrimaryKey::new();
        let expired_at = chrono::Utc::now() + chrono::Duration::days(90);
        invoke!(create_user_credential_hasher(&mut transaction, user_credential__has__hasher_pk, user_credential_pk, hasher_pk, hash, expired_at) else CreateUserCredentialHasher);

        let handle = Handle::new();
        invoke!(create_user_profile(&mut transaction, user_pk, handle, name) else CreateUserProfile);
        invoke!(create_user_agreement(&mut transaction, user_pk, agreements) else CreateUserAgreement);
        invoke!(complete_before_signup(&mut transaction, before_signup_pk) else CompleteBeforeSignup);

        transaction
            .commit()
            .await
            .map_err(ServiceError::DatabaseConnectionPool)?;

        Ok(())
    }
}
