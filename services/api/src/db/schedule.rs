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
                date,
                indicator,
                atoc_code,
                performance_monitoring
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (uid, identity, headcode, date, atoc_code) DO UPDATE SET
                uid = EXCLUDED.uid,
                identity = EXCLUDED.identity,
                headcode = EXCLUDED.headcode,
                date = EXCLUDED.date,
                indicator = EXCLUDED.indicator,
                atoc_code = EXCLUDED.atoc_code,
                performance_monitoring = EXCLUDED.performance_monitoring
            RETURNING
                id,
                uid,
                identity,
                headcode,
                date,
                indicator,
                atoc_code,
                performance_monitoring;",
        )
        .bind(schdle.uid)
        .bind(schdle.identity)
        .bind(schdle.headcode)
        .bind(schdle.date)
        .bind(schdle.indicator)
        .bind(schdle.atoc_code)
        .bind(schdle.performance_monitoring)
        .fetch_one(pool)
        .await?;
        debug!("Inserted Schedule with ID: {:?}", row.id);
        Ok(())
    }
}
