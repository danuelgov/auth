#![allow(unused)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

mod routes;

use rocket::Rocket;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rocket = Rocket::build();
    let rocket = routes::routes(rocket);
    rocket.launch().await?;

    Ok(())
}
