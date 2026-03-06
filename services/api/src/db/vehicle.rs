use log::debug;

pub use crate::db::schema::Vehicle;

impl Vehicle {
    pub async fn insert(
        pool: &sqlx::postgres::PgPool,
        vehicle: Vehicle,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
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
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
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
                max_speed = excluded.max_speed")
            .bind(vehicle.id)
            .bind(vehicle.livery)
            .bind(vehicle.decor)
            .bind(vehicle.vehicle_type)
            .bind(vehicle.specific_type)
            .bind(vehicle.resource_group_id)
            .bind(vehicle.resource_position)
            .bind(vehicle.planned_resource_group)
            .bind(vehicle.length_value)
            .bind(vehicle.length_measure)
            .bind(vehicle.weight)
            .bind(vehicle.special_characteristics)
            .bind(vehicle.seat_count)
            .bind(vehicle.cab_count)
            .bind(vehicle.date_entered_service)
            .bind(vehicle.date_registered)
            .bind(vehicle.category)
            .bind(vehicle.brake_type)
            .bind(vehicle.max_speed)
        .execute(pool)
        .await?;
        debug!("Upserted Vehicle with ID: {:?}", vehicle.id);
        Ok(())
    }
}
