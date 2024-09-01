use uuid::Uuid;
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, CommitMode, StreamConsumer};
use rdkafka::message::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client_id = format!("rust-basic-producer-{}", Uuid::new_v4());
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "my-rust-consumer")
        .set("bootstrap.servers", "localhost:29092")
        .set("auto.offset.reset", "earliest")
        .set("client.id", &client_id) // client.idを追加
        .create()?;

    consumer.subscribe(&["ticket-order"])?;

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                     m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                consumer.commit_message(&m, CommitMode::Async).unwrap();

            }
        }
    }
}
