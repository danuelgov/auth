mod repository;
mod request;
mod service;

use database_toolkit::DatabaseConnectionPool;
use repository::*;
use request::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

#[post("/signup", data = "<body>")]
pub async fn handler(
    pool: &State<DatabaseConnectionPool>,
    body: Json<Data>,
) -> Result<Status, Status> {
    let service = service(pool.inner().clone(), Repository, body);
    match service.execute().await {
        Ok(_) => Ok(Status::NoContent),
        Err(ServiceError::EmailAddressAlreadyExists) => Err(Status::Conflict),
        Err(ServiceError::NameAlreadyExists) => Err(Status::Conflict),
        Err(ServiceError::InvalidAgreement) => Err(Status::BadRequest),
        _ => Err(Status::InternalServerError),
    }
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
