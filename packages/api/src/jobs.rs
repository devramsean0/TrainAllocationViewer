use log::info;
use sqlx::sqlite;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn init_scheduler() -> anyhow::Result<()> {
    let mut sched = JobScheduler::new().await?;

    sched
        .add(Job::new_async("0 0 0 * * *", |uuid, _l| {
            Box::pin(async move {
                let pool: sqlite::SqlitePool =
                    sqlite::SqlitePool::connect("sqlite:data.db").await.unwrap();
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
