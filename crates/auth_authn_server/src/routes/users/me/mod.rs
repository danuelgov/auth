mod repository;
mod response;
mod service;

use auth_database::{user::columns::UserPrimaryKey, user_session::columns::UserSessionPrimaryKey};
use database_toolkit::DatabaseConnectionPool;
use guard::Session;
use repository::*;
use response::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

#[get("/internal/authn/users/me")]
pub async fn handler(
    session: Session,
    pool: &State<DatabaseConnectionPool>,
) -> Result<Response, Status> {
    let service = service(
        pool.inner().clone(),
        Repository,
        session.user_pk,
        session.user_session_pk,
    );
    match service.execute().await {
        Ok(response) => Ok(Response {
            body: Json(Body {
                user: User {
                    handle: response.user.handle,
                    name: response.user.name,
                    bio: response.user.bio,
                    image: response.user.image,
                },
            }),
        }),
        _ => Err(Status::InternalServerError),
    }
}

fn service(
    pool: DatabaseConnectionPool,
    repository: Repository,
    user_pk: UserPrimaryKey,
    user_session_pk: UserSessionPrimaryKey,
) -> Service<Repository> {
    Service {
        pool,
        repository,
        user_pk,
        user_session_pk,
    }
}
