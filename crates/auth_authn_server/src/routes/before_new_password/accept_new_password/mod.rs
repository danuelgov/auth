mod repository;
mod request;
mod service;

use database_toolkit::DatabaseConnectionPool;
use repository::*;
use request::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

use self::find_before_new_password::FindBeforeNewPasswordError;

#[post("/new_password/accept", data = "<body>")]
pub async fn handler(
    pool: &State<DatabaseConnectionPool>,
    body: Json<Data>,
) -> Result<Status, Status> {
    let service = service(pool.inner().clone(), Repository, body);
    match service.execute().await {
        Ok(_) => Ok(Status::Ok),
        Err(ServiceError::FindBeforeNewPassword(FindBeforeNewPasswordError::Used)) => {
            Ok(Status::Gone)
        }
        Err(ServiceError::FindBeforeNewPassword(FindBeforeNewPasswordError::Expired)) => {
            Ok(Status::Gone)
        }
        Err(ServiceError::FindBeforeNewPassword(FindBeforeNewPasswordError::NotFound)) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            dbg!(error);
            Err(Status::InternalServerError)
        }
    }
}

fn service(
    pool: DatabaseConnectionPool,
    repository: Repository,
    body: Json<Data>,
) -> Service<Repository> {
    Service {
        pool,
        repository,
        before_new_password_id: body.before_new_password_id,
        password: body.password.to_owned(),
    }
}
