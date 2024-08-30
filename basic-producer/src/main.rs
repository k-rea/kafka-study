use std::time::Duration;
use anyhow::Result;
use log::{error, info};
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct TicketOrder {
    order_id: String,
    user_id: String,
    content_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // TODO: retry, acksの設定
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("client.id", "basic-producer-1")
        .create()?;

    let topic = "ticket-order";

    let order_id = Uuid::new_v4().to_string();
    let user_id = "123";
    let content_id = "44444";

    let ticket_order = TicketOrder {
        order_id,
        user_id: user_id.to_string(),
        content_id: content_id.to_string()
    };

    let payload = serde_json::to_string(&ticket_order)?;

    info!("Sending message to topic {}", topic);

    let delivery_status = producer
        .send(
            FutureRecord::to(topic)
                .payload(&payload)
                .key(&ticket_order.user_id),
            Duration::from_secs(0),
        )
        .await;
    match delivery_status {
        Ok((partition, offset)) => {
            info!("Message sent successfully to partition {} at offset {}", partition, offset);
        }
        Err((error, _)) => {
            error!("Error while sending message: {:?}", error)
        }
    }
    Ok(())
}
