use log::info;
use sqlx::postgres::{self, PgConnectOptions, PgSslMode};
use std::str::FromStr;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn init_scheduler() -> anyhow::Result<()> {
    let mut sched = JobScheduler::new().await?;

    sched
        .add(Job::new_async("0 0 0 * * *", |uuid, _l| {
            Box::pin(async move {
                let database_url = std::env::var("DATABASE_URL").unwrap();
                let options = PgConnectOptions::from_str(&database_url)
                    .unwrap()
                    .ssl_mode(PgSslMode::Disable);
                let pool: postgres::PgPool = postgres::PgPool::connect_with(options).await.unwrap();
                info!("[{uuid} Running Corpus Update Job");

                crate::providers::corpus::update_corpus(&pool)
                    .await
                    .unwrap();
            })
        })?)
        .await?;

    sched.shutdown_on_ctrl_c();
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move { info!("Job Scheduler Shut Down") })
    }));

    sched.start().await?;

    Ok(())
}
