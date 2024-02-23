#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

mod routes;
mod states;

use rocket::Rocket;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let rocket = Rocket::build();
    let rocket = states::manage(rocket).await?;
    let rocket = routes::routes(rocket);
    rocket.launch().await?;

    Ok(())
}
