use crate::DatabaseConnectionPool;
use sqlx::{mysql::MySqlPoolOptions, Error};

#[derive(Clone)]
pub struct DatabaseConnectionPoolBuilder {
    max_connections: u32,
}

impl DatabaseConnectionPoolBuilder {
    #[inline]
    pub const fn new() -> Self {
        Self {
            max_connections: 10,
        }
    }

    #[inline]
    pub const fn max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = max_connections;
        self
    }

    #[inline]
    pub async fn build(self, url: &str) -> Result<DatabaseConnectionPool, Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(url)
            .await?;

        Ok(DatabaseConnectionPool(pool))
    }
}
