use log::debug;

pub use crate::db::schema::Vehicle;

impl Vehicle {
    pub async fn insert(
        pool: &sqlx::sqlite::SqlitePool,
        vehicle: Vehicle,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO vehicles (
                id,
                livery,
                decor,
                vehicle_type,
                specific_type,
                resource_group_id,
                resource_position,
                planned_resource_group,
                length_value,
                length_measure,
                weight,
                special_characteristics,
                seat_count,
                cab_count,
                date_entered_service,
                date_registered,
                category,
                brake_type,
                max_speed
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)
            ON CONFLICT(id) DO UPDATE SET
                livery = excluded.livery,
                decor = excluded.decor,
                vehicle_type = excluded.vehicle_type,
                specific_type = excluded.specific_type,
                resource_group_id = excluded.resource_group_id,
                resource_position = excluded.resource_position,
                planned_resource_group = excluded.planned_resource_group,
                length_value = excluded.length_value,
                length_measure = excluded.length_measure,
                weight = excluded.weight,
                special_characteristics = excluded.special_characteristics,
                seat_count = excluded.seat_count,
                cab_count = excluded.cab_count,
                date_entered_service = excluded.date_entered_service,
                date_registered = excluded.date_registered,
                category = excluded.category,
                brake_type = excluded.brake_type,
                max_speed = excluded.max_speed",
            vehicle.id,
            vehicle.livery,
            vehicle.decor,
            vehicle.vehicle_type,
            vehicle.specific_type,
            vehicle.resource_group_id,
            vehicle.resource_position,
            vehicle.planned_resource_group,
            vehicle.length_value,
            vehicle.length_measure,
            vehicle.weight,
            vehicle.special_characteristics,
            vehicle.seat_count,
            vehicle.cab_count,
            vehicle.date_entered_service,
            vehicle.date_registered,
            vehicle.category,
            vehicle.brake_type,
            vehicle.max_speed
        )
        .execute(pool)
        .await?;
        debug!("Upserted Vehicle with ID: {:?}", vehicle.id);
        Ok(())
    }
}
