mod repository;
mod request;
mod response;
mod service;

use database_toolkit::DatabaseConnectionPool;
use repository::*;
use request::*;
use response::*;
use rocket::{serde::json::Json, State};
use service::*;

#[post("/signup/accept", data = "<body>")]
pub async fn handler(pool: &State<DatabaseConnectionPool>, body: Json<Data>) -> Response {
    let service = service(pool.inner().clone(), Repository, body);
    let expected = service.execute().await;

    dbg!(&expected);

    Response {
        body: Json(Body {
            message: "Hello, world!".to_string(),
        }),
    }
}

fn service(
    pool: DatabaseConnectionPool,
    repository: Repository,
    body: Json<Data>,
) -> Service<Repository> {
    let Data { before_signup_id } = body.into_inner();

    Service {
        pool,
        repository,
        before_signup_id,
    }
}
