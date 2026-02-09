use log::debug;

pub use crate::db::schema::Allocation;

impl Allocation {
    pub async fn insert(
        pool: &sqlx::sqlite::SqlitePool,
        alloc: Allocation,
    ) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as!(
            Allocation,
            "INSERT INTO allocations (
                origin_datetime,
                origin_location,
                date,
                dest_location,
                dest_datetime,
                allocation_origin_datetime,
                allocation_origin_location,
                allocation_dest_datetime,
                allocation_dest_location
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            RETURNING id, origin_datetime, origin_location, date, dest_location, dest_datetime, allocation_origin_datetime, allocation_origin_location, allocation_dest_datetime, allocation_dest_location",
            alloc.origin_datetime,
            alloc.origin_location,
            alloc.date,
            alloc.dest_location,
            alloc.dest_datetime,
            alloc.allocation_origin_datetime,
            alloc.allocation_origin_location,
            alloc.allocation_dest_datetime,
            alloc.allocation_dest_location
        )
        .fetch_one(pool)
        .await?;
        debug!("Inserted Allocation with ID: {:?}", row.id);
        Ok(())
    }
}
