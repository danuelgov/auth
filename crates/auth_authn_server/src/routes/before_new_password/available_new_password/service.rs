use super::{
    exists_before_new_password_id::{Existence, ExistsBeforeNewPasswordIdError},
    RepositoryContract,
};
use auth_database::before_new_password::columns::BeforeNewPasswordIdentity;
use database_toolkit::DatabaseConnectionPool;

pub trait ServiceContract {
    async fn execute(&self) -> Result<Existence, ServiceError>;
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseConnectionPool(sqlx::Error),
    ExistsBeforeNewPasswordId(ExistsBeforeNewPasswordIdError),
}

pub struct Service<Repository: RepositoryContract> {
    pub pool: DatabaseConnectionPool,
    pub repository: Repository,
    pub before_new_password_id: BeforeNewPasswordIdentity,
}

impl<Repository: RepositoryContract> ServiceContract for Service<Repository> {
    async fn execute(&self) -> Result<Existence, ServiceError> {
        macro_rules! connection {
            () => {{
                self.pool
                    .connection()
                    .await
                    .map_err(ServiceError::DatabaseConnectionPool)?
            }};
        }

        let exists = self
            .repository
            .exists_before_new_password_id(connection!(), self.before_new_password_id)
            .await
            .map_err(ServiceError::ExistsBeforeNewPasswordId)?;

        Ok(exists)
    }
}
