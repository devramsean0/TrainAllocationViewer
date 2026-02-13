use log::debug;

pub use crate::db::schema::ResourceGroup;

impl ResourceGroup {
    pub async fn insert(
        pool: &sqlx::sqlite::SqlitePool,
        rsgrp: ResourceGroup,
    ) -> Result<ResourceGroup, sqlx::Error> {
        let row = sqlx::query_as::<_, ResourceGroup>(
            "INSERT INTO resource_groups (
                id,
                fleet,
                resource_type,
                status,
                end_of_day_miles
            ) VALUES (?1, ?2, ?3, ?4? ?5)
            ON CONFLICT(id) DO UPDATE SET
                fleet = excluded.fleet,
                resource_type = excluded.resource_type,
                status = excluded.status,
                end_of_day_miles = excluded.end_of_day_miles
            RETURNING id,
                fleet,
                resource_type,
                status,
                end_of_day_miles",
        )
        .bind(rsgrp.id)
        .bind(rsgrp.fleet)
        .bind(rsgrp.resource_type)
        .bind(rsgrp.status)
        .bind(rsgrp.end_of_day_miles)
        .fetch_one(pool)
        .await?;
        debug!("Upserted ResourceGroup with ID: {:?}", row.id);
        Ok(row)
    }
}
