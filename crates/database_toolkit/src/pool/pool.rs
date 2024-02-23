use crate::{DatabaseConnection, DatabaseConnectionPoolBuilder, Transaction};
use sqlx::{Error, MySqlPool};

#[derive(Clone)]
pub struct DatabaseConnectionPool(pub(crate) MySqlPool);

impl std::ops::Deref for DatabaseConnectionPool {
    type Target = MySqlPool;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DatabaseConnectionPool {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DatabaseConnectionPool {
    #[inline]
    pub const fn builder() -> DatabaseConnectionPoolBuilder {
        DatabaseConnectionPoolBuilder::new()
    }

    #[inline]
    pub async fn transaction(&self) -> Result<Transaction, Error> {
        let transaction = self.begin().await?;

        Ok(Transaction(transaction))
    }

    #[inline]
    pub async fn connection(&self) -> Result<DatabaseConnection, Error> {
        let connection = self.acquire().await?;

        Ok(DatabaseConnection(connection))
    }
}
