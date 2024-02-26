mod database;
mod ip_addr_rate_limit;

use rocket::{Build, Rocket};

#[derive(Debug)]
pub enum StateError {
    Database(database::DatabaseError),
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StateError {
    //
}

pub async fn manage(rocket: Rocket<Build>) -> Result<Rocket<Build>, StateError> {
    let rocket = database::manage(rocket)
        .await
        .map_err(StateError::Database)?;
    let rocket = ip_addr_rate_limit::manage(rocket).await;

    Ok(rocket)
}
