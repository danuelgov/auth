mod repository;
mod service;

use self::exists_before_new_password_id::Existence;
use auth_database::before_new_password::columns::BeforeNewPasswordIdentity;
use database_toolkit::DatabaseConnectionPool;
use repository::*;
use rocket::{http::Status, State};
use service::*;
use std::str::FromStr;

#[get("/new_password/available?<id>")]
pub async fn handler(pool: &State<DatabaseConnectionPool>, id: String) -> Result<Status, Status> {
    let Ok(before_new_password_id) = BeforeNewPasswordIdentity::from_str(&id) else {
        return Err(Status::BadRequest);
    };

    let service = service(pool.inner().clone(), Repository, before_new_password_id);
    match service.execute().await {
        Ok(Existence::Yes) => Ok(Status::Ok),
        Ok(Existence::No) => Ok(Status::NotFound),
        _ => Err(Status::InternalServerError),
    }
}

fn service(
    pool: DatabaseConnectionPool,
    repository: Repository,
    before_new_password_id: BeforeNewPasswordIdentity,
) -> Service<Repository> {
    Service {
        pool,
        repository,
        before_new_password_id,
    }
}
