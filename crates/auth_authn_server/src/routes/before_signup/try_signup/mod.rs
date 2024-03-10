mod repository;
mod request;
mod service;

use auth_event::EventClient;
use database_toolkit::DatabaseConnectionPool;
use guard::IpAddrRateLimit;
use repository::*;
use request::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

#[post("/signup", data = "<body>")]
pub async fn handler(
    _rate_limit: IpAddrRateLimit,
    pool: &State<DatabaseConnectionPool>,
    event_client: &State<EventClient>,
    body: Json<Data>,
) -> Result<Status, Status> {
    let service = service(
        pool.inner().clone(),
        Repository,
        event_client.inner().clone(),
        body,
    );
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
    event_client: EventClient,
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
        event_client,
        email_address,
        password,
        name,
        agreements,
    }
}
