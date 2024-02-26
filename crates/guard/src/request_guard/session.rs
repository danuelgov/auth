use crate::{Authorization, Bearer, Header};
use auth_database::{
    user::{self, columns::UserPrimaryKey},
    user_session::{
        self,
        columns::{UserSessionIdentity, UserSessionPrimaryKey},
        UserSession,
    },
};
use database_toolkit::{DatabaseConnectionPool, QueryBuilder};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};
use sqlx::FromRow;
use std::str::FromStr;

#[derive(Debug)]
pub struct Session {
    pub user_session_pk: UserSessionPrimaryKey,
    pub user_pk: UserPrimaryKey,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Outcome::Success(authorization) =
            request.guard::<Header<Authorization<Bearer>>>().await
        else {
            return Outcome::Forward(Status::Unauthorized);
        };
        let Ok(user_session_id) = UserSessionIdentity::from_str(authorization.as_str()) else {
            return Outcome::Forward(Status::Unauthorized);
        };
        let Outcome::Success(pool) = request.guard::<&State<DatabaseConnectionPool>>().await else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Ok(mut connection) = pool.connection().await else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Ok(row) = query(user_session_id)
            .build()
            .fetch_one(&mut *connection)
            .await
        else {
            return Outcome::Forward(Status::Unauthorized);
        };

        #[derive(FromRow)]
        struct Row {
            user_session_pk: Vec<u8>,
            user_pk: Vec<u8>,
            expired_at: chrono::DateTime<chrono::Utc>,
        }

        let Ok(Row {
            user_session_pk,
            user_pk,
            expired_at,
        }) = Row::from_row(&row)
        else {
            return Outcome::Forward(Status::Unauthorized);
        };
        let now = chrono::Utc::now();
        if expired_at < now {
            return Outcome::Forward(Status::Unauthorized);
        }
        let Ok(user_session_pk) = UserSessionPrimaryKey::try_from(user_session_pk) else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Ok(user_pk) = UserPrimaryKey::try_from(user_pk) else {
            return Outcome::Error((Status::InternalServerError, ()));
        };

        Outcome::Success(Session {
            user_session_pk,
            user_pk,
        })
    }
}

fn query<'q>(session_id: UserSessionIdentity) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .select(UserSession, |builder| {
            builder.columns(&[
                user_session::columns::USER_SESSION_PK,
                user_session::columns::USER_PK,
                user_session::columns::EXPIRED_AT,
            ])
        })
        .where_(|builder| {
            builder.condition(|builder| {
                let session_id: Vec<u8> = session_id.into();

                builder
                    .column(user_session::columns::ID)
                    .eq()
                    .value(session_id)
            })
        })
}
