#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

mod routes;
mod states;

use rocket::{Config, Rocket};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let rocket = Rocket::custom(Config {
        port: 8002,
        ..Default::default()
    });
    let rocket = states::manage(rocket).await?;
    let rocket = routes::routes(rocket);
    rocket.launch().await?;

    Ok(())
}
