mod me;

use rocket::{Build, Rocket};

pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![me::handler])
}
