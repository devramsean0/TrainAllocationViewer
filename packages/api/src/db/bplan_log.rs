use log::debug;

pub use crate::db::schema::BplanLog;

impl BplanLog {
    pub async fn get_by_creation_date(
        pool: &sqlx::postgres::PgPool,
        creation_date: String,
    ) -> Result<BplanLog, sqlx::Error> {
        let row = sqlx::query_as::<_, BplanLog>(
            "SELECT * FROM bplan_log
            WHERE
                creation_date = $1;",
        )
        .bind(creation_date)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    pub async fn insert(
        pool: &sqlx::postgres::PgPool,
        log: BplanLog,
    ) -> Result<BplanLog, sqlx::Error> {
        let row = sqlx::query_as::<_, BplanLog>(
            "INSERT INTO bplan_log (
                file_version,
                source_system,
                toc_id,
                timetable_start_date,
                timetable_end_date,
                cycle_type,
                cycle_stage,
                creation_date,
                sequence_number
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id,
                file_version,
                source_system,
                toc_id,
                timetable_start_date,
                timetable_end_date,
                cycle_type,
                cycle_stage,
                creation_date,
                sequence_number",
        )
        .bind(log.file_version)
        .bind(log.source_system)
        .bind(log.toc_id)
        .bind(log.timetable_start_date)
        .bind(log.timetable_end_date)
        .bind(log.cycle_type)
        .bind(log.cycle_stage)
        .bind(log.creation_date)
        .bind(log.sequence_number)
        .fetch_one(pool)
        .await?;
        debug!("Upserted AllocArchiveLog with ID: {:?}", row.id);
        Ok(row)
    }
}
