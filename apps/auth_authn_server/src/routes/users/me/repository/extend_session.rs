use auth_database::user_session::{self, columns::UserSessionPrimaryKey, UserSession};
use chrono::{DateTime, Utc};
use database_toolkit::{DatabaseConnection, QueryBuilder};

pub trait ExtendSessionContract {
    async fn extend_session(
        &self,
        connection: DatabaseConnection,
        user_session_pk: UserSessionPrimaryKey,
        expired_at: DateTime<Utc>,
    ) -> Result<(), ExtendSessionError>;
}

#[derive(Debug)]
pub enum ExtendSessionError {
    Database(sqlx::Error),
}

impl ExtendSessionContract for super::Repository {
    async fn extend_session(
        &self,
        mut connection: DatabaseConnection,
        user_session_pk: UserSessionPrimaryKey,
        expired_at: DateTime<Utc>,
    ) -> Result<(), ExtendSessionError> {
        query(user_session_pk, expired_at)
            .build()
            .execute(&mut *connection)
            .await
            .map_err(ExtendSessionError::Database)?;

        Ok(())
    }
}

fn query<'q>(
    user_session_pk: UserSessionPrimaryKey,
    expired_at: DateTime<Utc>,
) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .update(UserSession)
        .set(user_session::columns::EXPIRED_AT, expired_at)
        .where_(|builder| {
            builder.condition(|builder| {
                let user_session_pk: Vec<u8> = user_session_pk.into();

                builder
                    .column(user_session::columns::USER_SESSION_PK)
                    .eq()
                    .value(user_session_pk)
            })
        })
}
