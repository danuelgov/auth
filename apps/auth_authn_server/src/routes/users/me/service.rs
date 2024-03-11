use super::{
    extend_session::ExtendSessionError,
    find_user_profile_by_primary_key::FindUserProfileByPrimaryKeyError, RepositoryContract,
};
use auth_database::{user::columns::UserPrimaryKey, user_session::columns::UserSessionPrimaryKey};
use database_toolkit::DatabaseConnectionPool;

pub trait ServiceContract {
    async fn execute(&self) -> Result<Executed, ServiceError>;
}

pub struct Executed {
    pub user: UserEntity,
}

pub struct UserEntity {
    pub handle: String,
    pub name: String,
    pub bio: String,
    pub image: String,
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseConnectionPool(sqlx::Error),
    FindUserProfileByPrimaryKey(FindUserProfileByPrimaryKeyError),
    ExtendSession(ExtendSessionError),
}

pub struct Service<Repository: RepositoryContract> {
    pub pool: DatabaseConnectionPool,
    pub repository: Repository,
    pub user_session_pk: UserSessionPrimaryKey,
    pub user_pk: UserPrimaryKey,
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

        let user = self
            .repository
            .find_user_profile_by_primary_key(connection!(), self.user_pk)
            .await
            .map_err(ServiceError::FindUserProfileByPrimaryKey)?;

        let expired_at = chrono::Utc::now() + chrono::Duration::days(30);
        self.repository
            .extend_session(connection!(), self.user_session_pk, expired_at)
            .await
            .map_err(ServiceError::ExtendSession)?;

        Ok(Executed {
            user: UserEntity {
                handle: user.handle,
                name: user.name,
                bio: user.bio,
                image: user.image,
            },
        })
    }
}
