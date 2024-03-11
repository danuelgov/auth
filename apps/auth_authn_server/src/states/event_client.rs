use auth_event::EventClient;
use rocket::{Build, Rocket};

pub async fn manage(rocket: Rocket<Build>) -> Rocket<Build> {
    let event_client = EventClient::new().await;
    let rocket = rocket.manage(event_client);

    rocket
}
