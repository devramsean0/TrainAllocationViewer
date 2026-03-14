use log::debug;

pub use crate::db::schema::Schedule;

impl Schedule {
    pub async fn insert(
        pool: &sqlx::postgres::PgPool,
        schdle: Schedule,
    ) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as::<_, Schedule>(
            "INSERT INTO schedule (
                uid,
                identity,
                headcode,
                indicator,
                atoc_code,
                performance_monitoring,
                origin_location,
                dest_location,
                start_date,
                end_date
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (uid, identity, headcode, start_date, end_date, atoc_code) DO UPDATE SET
                uid = EXCLUDED.uid,
                identity = EXCLUDED.identity,
                headcode = EXCLUDED.headcode,
                indicator = EXCLUDED.indicator,
                atoc_code = EXCLUDED.atoc_code,
                performance_monitoring = EXCLUDED.performance_monitoring,
                origin_location = EXCLUDED.origin_location,
                dest_location = EXCLUDED.dest_location,
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date
            RETURNING
                id,
                uid,
                identity,
                headcode,
                indicator,
                atoc_code,
                performance_monitoring,
                origin_location,
                dest_location,
                start_date,
                end_date;",
        )
        .bind(schdle.uid)
        .bind(schdle.identity)
        .bind(schdle.headcode)
        .bind(schdle.indicator)
        .bind(schdle.atoc_code)
        .bind(schdle.performance_monitoring)
        .bind(schdle.origin_location)
        .bind(schdle.dest_location)
        .bind(schdle.start_date)
        .bind(schdle.end_date)
        .fetch_one(pool)
        .await?;
        debug!("Inserted Schedule with ID: {:?}", row.id);
        Ok(())
    }
}
