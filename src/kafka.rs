use anyhow::Result;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::StreamConsumer;

pub fn create_consumer() -> Result<StreamConsumer> {
    let host = std::env::var("KAFKA_HOST")?;
    let group = std::env::var("KAFKA_GROUP")?;
    let username = std::env::var("KAFKA_USERNAME")?;
    let password = std::env::var("KAFKA_PASSWORD")?;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", host)
        .set("group.id", group)
        .set("auto.offset.reset", "earliest")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanism", "PLAIN")
        .set("sasl.username", username)
        .set("sasl.password", password)
        .set("enable.auto.commit", "true")
        .create()?;

    Ok(consumer)
}
