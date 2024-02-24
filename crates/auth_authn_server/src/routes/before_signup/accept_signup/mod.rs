mod repository;
mod request;
mod service;

use self::find_before_signup::FindBeforeSignupError;
use database_toolkit::DatabaseConnectionPool;
use guard::IpAddrRateLimit;
use repository::*;
use request::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

#[post("/signup/accept", data = "<body>")]
pub async fn handler(
    _rate_limit: IpAddrRateLimit,
    pool: &State<DatabaseConnectionPool>,
    body: Json<Data>,
) -> Result<Status, Status> {
    let service = service(pool.inner().clone(), Repository, body);
    match service.execute().await {
        Ok(_) => Ok(Status::NoContent),
        Err(ServiceError::FindBeforeSignup(FindBeforeSignupError::NotFound)) => {
            Err(Status::NotFound)
        }
        Err(ServiceError::FindBeforeSignup(FindBeforeSignupError::Expired)) => Err(Status::Gone),
        _ => Err(Status::InternalServerError),
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
