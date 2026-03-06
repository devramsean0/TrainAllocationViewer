use std::time::Duration;

use futures::StreamExt;
use log::{debug, error, info};
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
    message::{Header, OwnedHeaders, ToBytes},
    producer::{FutureProducer, FutureRecord},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let source_kafka = std::env::var("UPSTREAM_KAFKA_HOST")?;
    let topic = std::env::var("KAFKA_TOPIC")?;
    let destination_kafka = std::env::var("DOWNSTREAM_KAFKA_HOST")?;

    // Authentication
    let group = std::env::var("KAFKA_GROUP")?;
    let username = std::env::var("KAFKA_USERNAME")?;
    let password = std::env::var("KAFKA_PASSWORD")?;

    info!(
        "Spinning up KafkaProxy with (source: {source_kafka}, destination: {destination_kafka}, topic: {topic})"
    );

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", source_kafka)
        .set("group.id", group)
        .set("auto.offset.reset", "earliest")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanism", "PLAIN")
        .set("sasl.username", username)
        .set("sasl.password", password)
        .set("enable.auto.commit", "true")
        .create()?;
    debug!("Created Consumer");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", destination_kafka)
        .set("message.timeout.ms", "5000")
        .set("broker.address.family", "v4")
        .create()?;

    debug!("Created Producer");

    consumer.subscribe(&[topic.as_str()])?;
    info!("KafkaProxy Spun Up");

    let mut stream = consumer.stream();

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => {
                debug!("Received Message");
                let key = message.key();
                let payload = message.payload();

                let mut record = FutureRecord::to(topic.as_str())
                    .payload(payload.unwrap_or_default().to_bytes());

                if let Some(k) = key {
                    record = record.key(k);
                }

                record = record.headers(OwnedHeaders::new().insert(Header {
                    key: "ProcessedBy",
                    value: Some("KafkaProxy"),
                }));

                match producer.send(record, Duration::from_secs(5)).await {
                    Ok(_) => {
                        //if let Some(k) = key {
                        //    info!("Message with key {} sent", String::from_utf8(k.to_vec())?);
                        //} else {
                        info!("Message Sent");
                        //}
                    }
                    Err(e) => {
                        error!("Error sending message: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error consuming Kafka: {}", e);
            }
        }
    }
    Ok(())
}
