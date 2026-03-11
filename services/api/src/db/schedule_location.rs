use log::debug;

pub use crate::db::schema::ScheduleLocation;

impl ScheduleLocation {
    pub async fn insert(
        pool: &sqlx::postgres::PgPool,
        loc: ScheduleLocation,
    ) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as::<_, ScheduleLocation>(
            "INSERT INTO schedule_locations (
                location,
                scheduled_departure_time,
                scheduled_arrival_time,
                scheduled_pass_time,
                public_departure_time,
                public_arrival_time,
                platform,
                line,
                engineering_allowance,
                pathing_allowance,
                performance_allowance,
                activity,
                schedule_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (schedule_id, location) DO UPDATE SET
                location = EXCLUDED.location,
                scheduled_departure_time = EXCLUDED.scheduled_departure_time,
                scheduled_arrival_time = EXCLUDED.scheduled_arrival_time,
                scheduled_pass_time = EXCLUDED.scheduled_pass_time,
                public_departure_time = EXCLUDED.public_departure_time,
                public_arrival_time = EXCLUDED.public_arrival_time,
                platform = EXCLUDED.platform,
                line = EXCLUDED.line,
                engineering_allowance = EXCLUDED.engineering_allowance,
                pathing_allowance = EXCLUDED.pathing_allowance,
                performance_allowance = EXCLUDED.performance_allowance,
                activity = EXCLUDED.activity,
                schedule_id = EXCLUDED.schedule_id
            RETURNING
                id,
                location,
                scheduled_departure_time,
                scheduled_arrival_time,
                scheduled_pass_time,
                public_departure_time,
                public_arrival_time,
                platform,
                line,
                engineering_allowance,
                pathing_allowance,
                performance_allowance,
                activity,
                schedule_id;",
        )
        .bind(loc.location)
        .bind(loc.scheduled_departure_time)
        .bind(loc.scheduled_arrival_time)
        .bind(loc.scheduled_pass_time)
        .bind(loc.public_departure_time)
        .bind(loc.public_arrival_time)
        .bind(loc.platform)
        .bind(loc.line)
        .bind(loc.engineering_allowance)
        .bind(loc.pathing_allowance)
        .bind(loc.performance_allowance)
        .bind(loc.activity)
        .bind(loc.schedule_id)
        .fetch_one(pool)
        .await?;
        debug!("Inserted Schedule Location with ID: {:?}", row.id);
        Ok(())
    }
}
