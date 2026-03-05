use log::info;
use sqlx::postgres::PgConnectOptions;
use std::str::FromStr;
use tokio::{signal, sync::broadcast};

use crate::jobs::init_scheduler;

mod db;
mod graphql;
mod jobs;
mod payload;
mod providers;
mod sources;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL")?;
    let options =
        PgConnectOptions::from_str(&database_url)?.ssl_mode(sqlx::postgres::PgSslMode::Disable);
    let pool: &'static sqlx::postgres::PgPool = Box::leak(Box::new(
        sqlx::postgres::PgPool::connect_with(options).await?,
    ));
    info!("Running database migrations...");
    sqlx::migrate!().run(pool).await?;
    info!("Database migrations completed successfully");
    providers::corpus::update_corpus(pool).await?;
    providers::bplan::update_bplan(pool).await?;

    init_scheduler().await?;

    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    if db::allocation::Allocation::count(pool).await? == 0
        || std::env::var("FORCE_ALLOC_ARCHIVE_UPDATE").unwrap_or_default() == "true"
    {
        info!("Updating Alloc Consist from archive");
        providers::alloc_consist_archive::download_archive(pool).await?;
    };

    providers::alloc_consist::init(&pool, &shutdown_tx).await?;
    graphql::serve(&pool, &shutdown_tx).await?;

    signal::ctrl_c().await.unwrap();
    info!("Main recieved Ctrl+C, Exiting");

    let _ = shutdown_tx.send(());
    Ok(())
}
