mod users;

use rocket::{Build, Rocket};

pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = users::routes(rocket);

    rocket
}
