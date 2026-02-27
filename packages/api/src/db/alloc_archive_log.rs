use log::debug;

pub use crate::db::schema::AllocArchiveLog;

impl AllocArchiveLog {
    pub async fn get_by_filename(
        pool: &sqlx::sqlite::SqlitePool,
        name: String,
    ) -> Result<AllocArchiveLog, sqlx::Error> {
        let row = sqlx::query_as::<_, AllocArchiveLog>(
            "SELECT * FROM alloc_archive_log
            WHERE
                file_name = ?1
            RETURNING id,
                file_name;",
        )
        .bind(name)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn insert(
        pool: &sqlx::sqlite::SqlitePool,
        log: AllocArchiveLog,
    ) -> Result<AllocArchiveLog, sqlx::Error> {
        let row = sqlx::query_as::<_, AllocArchiveLog>(
            "INSERT INTO alloc_archive_log (
                id,
                file_name
            ) VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET
                file_name = excluded.file_name
            RETURNING id,
                file_name",
        )
        .bind(log.id)
        .bind(log.file_name)
        .fetch_one(pool)
        .await?;
        debug!("Upserted AllocArchiveLog with ID: {:?}", row.id);
        Ok(row)
    }
}
