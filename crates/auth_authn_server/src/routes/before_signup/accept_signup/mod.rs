mod repository;
mod service;

use self::find_before_signup::FindBeforeSignupError;
use auth_database::before_signup::columns::BeforeSignupIdentity;
use auth_event::EventClient;
use database_toolkit::DatabaseConnectionPool;
use guard::{Authorization, Bearer, Header, IpAddrRateLimit};
use repository::*;
use rocket::{http::Status, State};
use service::*;

#[post("/signup", rank = 1)]
pub async fn handler(
    _rate_limit: IpAddrRateLimit,
    pool: &State<DatabaseConnectionPool>,
    event_client: &State<EventClient>,
    authorization: Header<Authorization<Bearer>>,
) -> Result<Status, Status> {
    let before_signup_id = match authorization.value().as_str().parse() {
        Ok(before_signup_id) => before_signup_id,
        Err(_) => return Err(Status::Forbidden),
    };
    let service = service(
        pool.inner().clone(),
        Repository,
        event_client.inner().clone(),
        before_signup_id,
    );
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
    event_client: EventClient,
    before_signup_id: BeforeSignupIdentity,
) -> Service<Repository> {
    Service {
        pool,
        repository,
        event_client,
        before_signup_id,
    }
}
