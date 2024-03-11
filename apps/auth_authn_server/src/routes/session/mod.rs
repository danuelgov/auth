mod login;
mod otp;

use rocket::{Build, Rocket};

pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![login::handler, otp::handler])
}
