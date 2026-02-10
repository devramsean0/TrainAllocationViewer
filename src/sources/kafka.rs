use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use sqlx::{Pool, Sqlite};

type CallbackFn = fn(
    m: Vec<u8>,
    pool: &'static Pool<Sqlite>,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'static>,
>;

pub struct KafkaClient {
    consumer: StreamConsumer,
    callback: CallbackFn,
}

impl KafkaClient {
    pub fn new(callback: CallbackFn) -> anyhow::Result<Self> {
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

        Ok(Self { consumer, callback })
    }

    pub fn subscribe(&mut self, topics: &[&str]) -> anyhow::Result<()> {
        self.consumer.subscribe(topics)?;

        Ok(())
    }

    pub async fn recv(&self, pool: &'static Pool<Sqlite>) -> anyhow::Result<()> {
        match self.consumer.recv().await {
            Err(e) => {
                eprintln!("Kafka error: {}", e);
                Ok(())
            }
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    (self.callback)(payload.to_vec(), pool).await?;
                }
                Ok(())
            }
        }
    }
}
