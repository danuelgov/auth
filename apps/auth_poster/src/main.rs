#![allow(clippy::all)]

#[macro_use]
extern crate serde;

mod email;
mod event;

use aws_lambda_events::sqs::{BatchItemFailure, SqsBatchResponse, SqsEvent};
use email::*;
use event::*;
use lambda_runtime::{run, service_fn, LambdaEvent};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let email_client = EmailClient::new().await;
    run(service_fn(|event| handler(&email_client, event))).await?;

    Ok(())
}

async fn handler(
    email_client: &EmailClient,
    event: LambdaEvent<SqsEvent>,
) -> Result<SqsBatchResponse, Box<dyn std::error::Error>> {
    let mut tasks = JoinSet::new();
    for record in event.payload.records {
        let (Some(message_id), Some(body)) = (record.message_id, record.body) else {
            continue;
        };
        let email_client = email_client.clone();
        tasks.spawn(async move {
            if let Ok(event) = serde_json::from_str(&body) {
                if execute(event, &email_client).await.is_err() {
                    return Err(message_id);
                }
            };
            Ok(())
        });
    }

    let mut batch_item_failures = vec![];
    while let Some(executed) = tasks.join_next().await {
        if let Ok(Err(item_identifier)) = executed {
            batch_item_failures.push(BatchItemFailure { item_identifier });
        }
    }

    Ok(SqsBatchResponse {
        batch_item_failures,
    })
}
