use sqlx::MySql;

type MySqlTransaction = sqlx::Transaction<'static, MySql>;

pub struct Transaction(pub(crate) MySqlTransaction);

impl std::ops::Deref for Transaction {
    type Target = sqlx::MySqlConnection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::ops::DerefMut for Transaction {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl Transaction {
    #[inline]
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.0.commit().await
    }

    #[inline]
    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.0.rollback().await
    }
}
