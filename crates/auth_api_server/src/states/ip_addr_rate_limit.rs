use guard::IpAddrRateLimiter;
use rocket::{Build, Rocket};

pub async fn manage(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.manage(IpAddrRateLimiter::new(100, 60))
}
