use log::info;
use tokio::signal;

mod db;
mod payload;
mod providers;
mod sources;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool: &'static sqlx::sqlite::SqlitePool = Box::leak(Box::new(
        sqlx::sqlite::SqlitePool::connect("sqlite:data.db").await?,
    ));
    sqlx::migrate!().run(pool).await?;

    providers::corpus::update_corpus(pool).await?;

    let mut kafka = sources::kafka::KafkaClient::new(providers::alloc_consist::callback)?;
    kafka.subscribe(&["prod-1033-Passenger-Train-Allocation-and-Consist-1_0"])?;

    info!("Listening for messages…");

    loop {
        tokio::select! {
            _msg = kafka.recv(pool) => {},
            _ = signal::ctrl_c() => {
                println!("Shutting down");
                break;
            }
        }
    }
    Ok(())
}
