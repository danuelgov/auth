mod accept_new_password;
mod available_new_password;
mod try_new_password;

use rocket::{Build, Rocket};

pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/",
        routes![
            accept_new_password::handler,
            available_new_password::handler,
            try_new_password::handler
        ],
    )
}
