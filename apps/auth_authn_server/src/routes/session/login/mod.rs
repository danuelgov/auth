mod repository;
mod response;
mod service;

use auth_event::EventClient;
use database_toolkit::DatabaseConnectionPool;
use guard::{Authorization, Basic, Header, IpAddrRateLimit};
use new_type::{EmailAddress, IpAddr, Password};
use repository::*;
use response::*;
use rocket::{http::Status, serde::json::Json, State};
use service::*;

#[post("/login")]
pub async fn handler(
    _rate_limit: IpAddrRateLimit,
    ip_addr: IpAddr,
    pool: &State<DatabaseConnectionPool>,
    event_client: &State<EventClient>,
    authorization: Header<Authorization<Basic>>,
) -> Result<Response, Status> {
    let Ok(email_address) = authorization.id().parse() else {
        return Err(Status::Unauthorized);
    };
    let Ok(password) = authorization.password().parse() else {
        return Err(Status::Unauthorized);
    };
    let service = service(
        pool.inner().clone(),
        Repository,
        event_client.inner().clone(),
        ip_addr,
        email_address,
        password,
    );
    match service.execute().await {
        Ok(response) => Ok(Response {
            body: Json(Body {
                token: response.user_session_id.to_string(),
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
    event_client: EventClient,
    ip_address: IpAddr,
    email_address: EmailAddress,
    password: Password,
) -> Service<Repository> {
    Service {
        pool,
        repository,
        event_client,
        email_address,
        password,
        ip_address,
    }
}
