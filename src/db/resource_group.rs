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
                fleet
            ) VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET
                fleet = excluded.fleet
            RETURNING id, fleet",
        )
        .bind(rsgrp.id)
        .bind(rsgrp.fleet)
        .fetch_one(pool)
        .await?;
        debug!("Upserted ResourceGroup with ID: {:?}", row.id);
        Ok(row)
    }
}
