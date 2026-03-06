use log::debug;

pub use crate::db::schema::AllocArchiveLog;

impl AllocArchiveLog {
    pub async fn get_by_filename(
        pool: &sqlx::postgres::PgPool,
        name: String,
    ) -> Result<AllocArchiveLog, sqlx::Error> {
        let row = sqlx::query_as::<_, AllocArchiveLog>(
            "SELECT * FROM alloc_archive_log
            WHERE
                file_name = $1;",
        )
        .bind(name)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn insert(
        pool: &sqlx::postgres::PgPool,
        log: AllocArchiveLog,
    ) -> Result<AllocArchiveLog, sqlx::Error> {
        let row = sqlx::query_as::<_, AllocArchiveLog>(
            "INSERT INTO alloc_archive_log (
                file_name
            ) VALUES ($1)
            ON CONFLICT(file_name) DO UPDATE SET
                file_name = excluded.file_name
            RETURNING id,
                file_name",
        )
        .bind(log.file_name)
        .fetch_one(pool)
        .await?;
        debug!("Upserted AllocArchiveLog with ID: {:?}", row.id);
        Ok(row)
    }
}
