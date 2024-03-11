use auth_database::{
    user::columns::UserPrimaryKey,
    user_session::{
        self,
        columns::{UserSessionExpiredAt, UserSessionIdentity, UserSessionPrimaryKey},
        UserSession,
    },
};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use new_type::IpAddr;

pub trait CreateSessionContract {
    async fn create_session(
        &self,
        connection: DatabaseConnection,
        user_session_pk: UserSessionPrimaryKey,
        user_session_id: UserSessionIdentity,
        user_pk: UserPrimaryKey,
        ip_address: IpAddr,
        expired_at: UserSessionExpiredAt,
    ) -> Result<(), CreateSessionError>;
}

#[derive(Debug)]
pub enum CreateSessionError {
    Database(sqlx::Error),
}

impl CreateSessionContract for super::Repository {
    async fn create_session(
        &self,
        mut connection: DatabaseConnection,
        user_session_pk: UserSessionPrimaryKey,
        user_session_id: UserSessionIdentity,
        user_pk: UserPrimaryKey,
        ip_address: IpAddr,
        expired_at: UserSessionExpiredAt,
    ) -> Result<(), CreateSessionError> {
        query(
            user_session_pk,
            user_session_id,
            user_pk,
            ip_address,
            expired_at,
        )
        .build()
        .execute(&mut *connection)
        .await
        .map_err(CreateSessionError::Database)?;

        Ok(())
    }
}

fn query<'q>(
    user_session_pk: UserSessionPrimaryKey,
    user_session_id: UserSessionIdentity,
    user_pk: UserPrimaryKey,
    ip_address: IpAddr,
    expired_at: UserSessionExpiredAt,
) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .insert_into(
            UserSession,
            &[
                user_session::columns::USER_SESSION_PK,
                user_session::columns::ID,
                user_session::columns::USER_PK,
                user_session::columns::IP_ADDRESS,
                user_session::columns::EXPIRED_AT,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let user_session_pk: Vec<u8> = user_session_pk.into();
                let user_session_id: Vec<u8> = user_session_id.into();
                let user_pk: Vec<u8> = user_pk.into();
                let ip_address: Vec<u8> = ip_address.into();
                let expired_at = expired_at.naive_utc();

                builder
                    .separated(
                        [user_session_pk, user_session_id, user_pk, ip_address].into_iter(),
                        |builder, value| builder.value(value),
                    )
                    .comma()
                    .value(expired_at)
            })
        })
}
