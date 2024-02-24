mod request;

use guard::IpAddrRateLimit;
use request::*;
use rocket::serde::json::Json;

#[post("/login", data = "<body>")]
pub async fn handler(_rate_limit: IpAddrRateLimit, body: Json<Data>) {
    dbg!(&body);
}
