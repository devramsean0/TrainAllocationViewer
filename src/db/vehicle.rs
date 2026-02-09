use log::debug;

pub use crate::db::schema::Vehicle;

impl Vehicle {
    pub async fn insert(
        pool: &sqlx::sqlite::SqlitePool,
        vehicle: Vehicle,
    ) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as!(
            Vehicle,
            "INSERT INTO vehicles (
                id,
                livery,
                decor,
                vehicle_type,
                specific_type
            ) VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                livery = excluded.livery,
                decor = excluded.decor,
                vehicle_type = excluded.vehicle_type,
                specific_type = excluded.specific_type
            RETURNING id, livery, decor, vehicle_type, specific_type",
            vehicle.id,
            vehicle.livery,
            vehicle.decor,
            vehicle.vehicle_type,
            vehicle.specific_type
        )
        .fetch_one(pool)
        .await?;
        debug!("Upserted Vehicle with ID: {:?}", row.id);
        Ok(())
    }
}
