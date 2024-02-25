mod repository;
mod request;
mod response;
mod service;

use database_toolkit::DatabaseConnectionPool;
use guard::IpAddrRateLimit;
use new_type::IpAddr;
use repository::*;
use request::*;
use response::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

#[post("/login", data = "<body>")]
pub async fn handler(
    _rate_limit: IpAddrRateLimit,
    ip_addr: IpAddr,
    pool: &State<DatabaseConnectionPool>,
    body: Json<Data>,
) -> Result<Response, Status> {
    let service = service(pool.inner().clone(), Repository, ip_addr, body);
    match service.execute().await {
        Ok(response) => Ok(Response {
            body: Json(Body {
                expired_password: response.expired_password,
            }),
        }),
        Err(ServiceError::FindPasswordByEmailAddress(_)) => Err(Status::Unauthorized),
        Err(ServiceError::PasswordMismatch) => Err(Status::Unauthorized),
        _ => Err(Status::InternalServerError),
    }
}

fn service(
    pool: DatabaseConnectionPool,
    repository: Repository,
    ip_address: IpAddr,
    body: Json<Data>,
) -> Service<Repository> {
    let Data {
        email_address,
        password,
    } = body.into_inner();

    Service {
        pool,
        repository,
        email_address,
        password,
        ip_address,
    }
}
