use super::{
    create_session::CreateSessionError, find_user_by_email_address::FindUserByEmailAddressError,
    send_event::SendEventError, RepositoryContract,
};
use crate::routes::session::login::find_user_by_email_address::FindUserByEmailAddressEntity;
use auth_database::{
    hasher,
    user_session::columns::{UserSessionIdentity, UserSessionPrimaryKey},
};
use auth_event::EventClient;
use database_toolkit::DatabaseConnectionPool;
use new_type::{EmailAddress, Hasher, HasherError, IpAddr, Password};

pub trait ServiceContract {
    async fn execute(&self) -> Result<Executed, ServiceError>;
}

pub struct Executed {
    pub expired_password: bool,
}

pub struct Service<Repository: RepositoryContract> {
    pub pool: DatabaseConnectionPool,
    pub repository: Repository,
    pub event_client: EventClient,
    pub email_address: EmailAddress,
    pub password: Password,
    pub ip_address: IpAddr,
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseConnectionPool(sqlx::Error),
    FindPasswordByEmailAddress(FindUserByEmailAddressError),
    Verify(HasherError),
    PasswordMismatch,
    CreateSession(CreateSessionError),
    SendEvent(SendEventError),
}

impl<Repository: RepositoryContract> ServiceContract for Service<Repository> {
    async fn execute(&self) -> Result<Executed, ServiceError> {
        macro_rules! connection {
            () => {
                self.pool
                    .connection()
                    .await
                    .map_err(ServiceError::DatabaseConnectionPool)?
            };
        }

        let FindUserByEmailAddressEntity {
            user_pk,
            hasher_pk,
            hash,
            expired_at,
        } = self
            .repository
            .find_password_by_email_address(connection!(), self.email_address.clone())
            .await
            .map_err(ServiceError::FindPasswordByEmailAddress)?;
        let hasher = match hasher_pk.known_kind() {
            hasher::KnownKind::Argon2 => Hasher::Argon2,
            hasher::KnownKind::Bcrypt => Hasher::Bcrypt,
        };
        if !self
            .password
            .verify(hasher, hash.as_ref())
            .await
            .map_err(ServiceError::Verify)?
        {
            return Err(ServiceError::PasswordMismatch);
        }

        let now = chrono::Utc::now().naive_utc();
        {
            let user_session_pk = UserSessionPrimaryKey::new();
            let user_session_id = UserSessionIdentity::new();
            let expired_at = now + chrono::Duration::days(30);
            self.repository
                .create_session(
                    connection!(),
                    user_session_pk,
                    user_session_id,
                    user_pk,
                    self.ip_address,
                    expired_at.into(),
                )
                .await
                .map_err(ServiceError::CreateSession)?;
        }

        self.repository
            .send_event(
                self.event_client.clone(),
                user_pk,
                self.email_address.clone(),
            )
            .await
            .map_err(ServiceError::SendEvent)?;

        Ok(Executed {
            expired_password: expired_at.naive_utc() < now,
        })
    }
}
