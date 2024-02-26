mod before_new_password;
mod before_signup;
mod session;

use rocket::{Build, Rocket};

pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = before_new_password::routes(rocket);
    let rocket = before_signup::routes(rocket);
    let rocket = session::routes(rocket);

    rocket
}
