mod user;

use rocket::{Build, Rocket};

pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = user::routes(rocket);

    rocket
}
