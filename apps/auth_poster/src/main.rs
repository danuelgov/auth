#[macro_use]
extern crate serde;

mod email;
mod event;
mod generated;
mod poster;

use email::*;
use event::*;
use generated::*;
use poster::*;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Event(EventError),
}

#[tokio::main]
async fn main() {
    let poster_client = PosterClient::new(QUEUE_URL).await;
    let email_client = EmailClient::new().await;

    loop {
        let Ok(messages) = poster_client.receive().await else {
            continue;
        };

        for message in messages {
            if let Some(body) = message.body {
                if let Err(error) = post(&email_client, &body).await {
                    dbg!(error);
                    continue;
                }
            }
            if let Some(receipt_handle) = message.receipt_handle {
                let _ = poster_client.commit(&receipt_handle).await;
            }
        }
    }
}

async fn post(email_client: &EmailClient, body: &str) -> Result<(), Error> {
    let event: Event = serde_json::from_str(&body).map_err(Error::Json)?;
    event.execute(email_client).await.map_err(Error::Event)?;

    Ok(())
}
