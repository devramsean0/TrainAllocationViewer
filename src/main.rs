use log::info;
use tokio::{signal, sync::broadcast};

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

    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    providers::alloc_consist::init(&pool, &shutdown_tx).await?;

    signal::ctrl_c().await.unwrap();
    info!("Main recieved Ctrl+C, Exiting");

    let _ = shutdown_tx.send(());
    Ok(())
}
