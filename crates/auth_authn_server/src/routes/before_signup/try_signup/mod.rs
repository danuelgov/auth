mod repository;
mod request;
mod service;

use database_toolkit::DatabaseConnectionPool;
use repository::*;
use request::*;
use rocket::{serde::json::Json, State};
use service::*;

#[post("/signup", data = "<body>")]
pub async fn handler(pool: &State<DatabaseConnectionPool>, body: Json<Data>) {
    let service = service(pool.inner().clone(), Repository, body);
    let expected = service.execute().await;

    dbg!(&expected);
}

fn service(
    pool: DatabaseConnectionPool,
    repository: Repository,
    body: Json<Data>,
) -> Service<Repository> {
    let Data {
        email_address,
        password,
        name,
        agreements,
    } = body.into_inner();

    Service {
        pool,
        repository,
        email_address,
        password,
        name,
        agreements,
    }
}
