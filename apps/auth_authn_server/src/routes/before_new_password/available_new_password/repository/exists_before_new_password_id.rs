use auth_database::before_new_password::{
    self, columns::BeforeNewPasswordIdentity, BeforeNewPassword,
};
use chrono::{DateTime, Utc};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use sqlx::FromRow;

pub trait ExistsBeforeNewPasswordIdContract {
    async fn exists_before_new_password_id(
        &self,
        connection: DatabaseConnection,
        before_new_password_id: BeforeNewPasswordIdentity,
        now: DateTime<Utc>,
    ) -> Result<(), ExistsBeforeNewPasswordIdError>;
}

#[derive(Debug)]
pub enum ExistsBeforeNewPasswordIdError {
    Database(sqlx::Error),
    Row(sqlx::Error),
    Used,
    Expired,
    NotFound,
}

impl ExistsBeforeNewPasswordIdContract for super::Repository {
    async fn exists_before_new_password_id(
        &self,
        mut connection: DatabaseConnection,
        before_new_password_id: BeforeNewPasswordIdentity,
        now: DateTime<Utc>,
    ) -> Result<(), ExistsBeforeNewPasswordIdError> {
        let row = query(before_new_password_id)
            .build()
            .fetch_optional(&mut *connection)
            .await
            .map_err(ExistsBeforeNewPasswordIdError::Database)?
            .ok_or(ExistsBeforeNewPasswordIdError::NotFound)?;

        #[derive(FromRow)]
        struct Row {
            expired_at: DateTime<Utc>,
            completed_at: DateTime<Utc>,
        }

        match Row::from_row(&row).map_err(ExistsBeforeNewPasswordIdError::Row)? {
            row if row.completed_at < now => Err(ExistsBeforeNewPasswordIdError::Used),
            row if row.expired_at < now => Err(ExistsBeforeNewPasswordIdError::Expired),
            _ => Ok(()),
        }
    }
}

fn query<'q>(before_new_password_id: BeforeNewPasswordIdentity) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .select(BeforeNewPassword, |builder| {
            builder.columns(&[
                before_new_password::columns::EXPIRED_AT,
                before_new_password::columns::COMPLETED_AT,
            ])
        })
        .where_(|builder| {
            builder.condition(|builder| {
                let before_new_password_id: Vec<_> = before_new_password_id.into();

                builder
                    .column(before_new_password::columns::ID)
                    .eq()
                    .value(before_new_password_id)
            })
        })
}
