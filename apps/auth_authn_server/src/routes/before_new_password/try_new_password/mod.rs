mod repository;
mod request;
mod service;

use self::find_user_by_email_address::FindUserByEmailAddressError;
use database_toolkit::DatabaseConnectionPool;
use repository::*;
use request::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

#[post("/new_password", data = "<body>")]
pub async fn handler(
    pool: &State<DatabaseConnectionPool>,
    body: Json<Data>,
) -> Result<Status, Status> {
    let service = service(pool.inner().clone(), Repository, body);
    match service.execute().await {
        Ok(_) => Ok(Status::NoContent),
        Err(ServiceError::FindUserByEmailAddress(FindUserByEmailAddressError::NotFound)) => {
            Ok(Status::NotFound)
        }
        _ => Err(Status::InternalServerError),
    }
}

fn service(
    pool: DatabaseConnectionPool,
    repository: Repository,
    body: Json<Data>,
) -> Service<Repository> {
    let Data { email_address } = body.into_inner();

    Service {
        pool,
        repository,
        email_address,
    }
}
