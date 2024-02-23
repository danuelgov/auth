use sqlx::{pool::PoolConnection, MySql};

pub(crate) type MySqlConnection = PoolConnection<MySql>;

pub struct DatabaseConnection(pub(crate) MySqlConnection);

impl std::ops::Deref for DatabaseConnection {
    type Target = sqlx::MySqlConnection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DatabaseConnection {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
