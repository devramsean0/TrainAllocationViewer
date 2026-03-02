use log::{debug, info};

pub use crate::db::schema::Allocation;

impl Allocation {
    pub async fn insert(
        pool: &sqlx::postgres::PgPool,
        alloc: Allocation,
    ) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as::<_, Allocation>(
            "INSERT INTO allocations (
                origin_datetime,
                origin_location,
                origin_country_code_iso,
                origin_subsidiary_information_code,
                origin_subsidiary_information_company,
                date,
                dest_location,
                dest_country_code_iso,
                dest_subsidiary_information_code,
                dest_subsidiary_information_company,
                dest_datetime,
                allocation_origin_datetime,
                allocation_origin_location,
                allocation_origin_country_code_iso,
                allocation_origin_subsidiary_information_code,
                allocation_origin_subsidiary_information_company,
                allocation_dest_datetime,
                allocation_dest_location,
                allocation_dest_country_code_iso,
                allocation_dest_subsidiary_information_code,
                allocation_dest_subsidiary_information_company,
                resource_group_id,
                sequence_number,
                resource_group_position,
                diagram_no,
                origin_miles,
                destination_miles,
                reversed
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)
            RETURNING
                id,
                origin_datetime,
                origin_location,
                origin_country_code_iso,
                origin_subsidiary_information_code,
                origin_subsidiary_information_company,
                date,
                dest_location,
                dest_country_code_iso,
                dest_subsidiary_information_code,
                dest_subsidiary_information_company,
                dest_datetime,
                allocation_origin_datetime,
                allocation_origin_location,
                allocation_origin_country_code_iso,
                allocation_origin_subsidiary_information_code,
                allocation_origin_subsidiary_information_company,
                allocation_dest_datetime,
                allocation_dest_location,
                allocation_dest_country_code_iso,
                allocation_dest_subsidiary_information_code,
                allocation_dest_subsidiary_information_company,
                resource_group_id,
                sequence_number,
                resource_group_position,
                diagram_no,
                origin_miles,
                destination_miles,
                reversed")
            .bind(alloc.origin_datetime)
            .bind(alloc.origin_location)
            .bind(alloc.origin_country_code_iso)
            .bind(alloc.origin_subsidiary_information_code)
            .bind(alloc.origin_subsidiary_information_company)
            .bind(alloc.date)
            .bind(alloc.dest_location)
            .bind(alloc.dest_country_code_iso)
            .bind(alloc.dest_subsidiary_information_code)
            .bind(alloc.dest_subsidiary_information_company)
            .bind(alloc.dest_datetime)
            .bind(alloc.allocation_origin_datetime)
            .bind(alloc.allocation_origin_location)
            .bind(alloc.allocation_origin_country_code_iso)
            .bind(alloc.allocation_origin_subsidiary_information_code)
            .bind(alloc.allocation_origin_subsidiary_information_company)
            .bind(alloc.allocation_dest_datetime)
            .bind(alloc.allocation_dest_location)
            .bind(alloc.allocation_dest_country_code_iso)
            .bind(alloc.allocation_dest_subsidiary_information_code)
            .bind(alloc.allocation_dest_subsidiary_information_company)
            .bind(alloc.resource_group_id)
            .bind(alloc.sequence_number)
            .bind(alloc.resource_group_position)
            .bind(alloc.diagram_no)
            .bind(alloc.origin_miles)
            .bind(alloc.destination_miles)
            .bind(alloc.reversed)
        .fetch_one(pool)
        .await?;
        debug!("Inserted Allocation with ID: {:?}", row.id);
        Ok(())
    }

    pub async fn count(pool: &sqlx::postgres::PgPool) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM allocations;")
            .fetch_one(pool)
            .await?;

        info!("Counted allocations: {count}");
        Ok(count)
    }
}
