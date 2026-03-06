use log::debug;

pub use crate::db::schema::CifScheduleLog;

impl CifScheduleLog {
    pub async fn insert(
        pool: &sqlx::postgres::PgPool,
        log: CifScheduleLog,
    ) -> Result<CifScheduleLog, sqlx::Error> {
        let row = sqlx::query_as::<_, CifScheduleLog>(
            "INSERT INTO cif_schedule_log (
                mainframe_identity,
                extract_date,
                extract_time,
                file_reference,
                version
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING 
                id,
                mainframe_identity,
                extract_date,
                extract_time,
                file_reference,
                version",
        )
        .bind(log.mainframe_identity)
        .bind(log.extract_date)
        .bind(log.extract_time)
        .bind(log.file_reference)
        .bind(log.version)
        .fetch_one(pool)
        .await?;
        debug!("Upserted CifScheduleLog with ID: {:?}", row.id);
        Ok(row)
    }
}
