use crate::{RateLimit, RateLimiter};
use new_type::IpAddr;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};

pub struct IpAddrRateLimiter(RateLimiter<IpAddr>);

#[derive(Debug, Clone)]
pub struct IpAddrRateLimit(pub(crate) RateLimit);

impl std::ops::Deref for IpAddrRateLimiter {
    type Target = RateLimiter<IpAddr>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IpAddrRateLimit {
    type Error = ();

    #[inline]
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(rate_limiter) = request
            .guard::<&State<IpAddrRateLimiter>>()
            .await
            .succeeded()
        else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Some(ip) = request.client_ip().map(IpAddr::from) else {
            return Outcome::Error((Status::Forbidden, ()));
        };
        let rate_limit = rate_limiter.increment(ip).await;
        if rate_limit.available() {
            Outcome::Success(IpAddrRateLimit(rate_limit))
        } else {
            Outcome::Error((Status::TooManyRequests, ()))
        }
    }
}

impl std::ops::Deref for IpAddrRateLimit {
    type Target = RateLimit;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IpAddrRateLimiter {
    #[inline]
    pub fn new(max_requests: u32, duration: u32) -> Self {
        IpAddrRateLimiter(RateLimiter::new(max_requests, duration))
    }
}
