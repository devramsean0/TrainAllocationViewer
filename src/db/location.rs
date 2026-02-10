use log::debug;

use crate::corpus::LocationEntry;
pub use crate::db::schema::Location;

impl Location {
    pub async fn insert(pool: &sqlx::sqlite::SqlitePool, loc: Location) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as!(
            Location,
            "INSERT INTO locations (
                nlc,
                stanox,
                tiploc,
                crs,
                uic,
                nlcdesc,
                axis,
                nlcdesc16
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            RETURNING id, nlc, stanox, tiploc, crs, uic, nlcdesc, axis, nlcdesc16",
            loc.nlc,
            loc.stanox,
            loc.tiploc,
            loc.crs,
            loc.uic,
            loc.nlcdesc,
            loc.axis,
            loc.nlcdesc16
        )
        .fetch_one(pool)
        .await?;
        debug!("Inserted Allocation with ID: {:?}", row.id);
        Ok(())
    }

    pub async fn insert_bulk(
        pool: &sqlx::sqlite::SqlitePool,
        locs: &[LocationEntry],
    ) -> Result<(), sqlx::Error> {
        if locs.is_empty() {
            return Ok(());
        }

        for chunk in locs.chunks(1000) {
            let mut builder = sqlx::QueryBuilder::new(
                "INSERT INTO locations (nlc, stanox, tiploc, crs, uic, nlcdesc, axis, nlcdesc16) ",
            );

            builder.push_values(chunk, |mut b, loc| {
                b.push_bind(&loc.nlc);
                b.push_bind(&loc.stanox);
                b.push_bind(&loc.tiploc);
                b.push_bind(&loc.crs);
                b.push_bind(&loc.uic);
                b.push_bind(&loc.nlcdesc);
                b.push_bind(&loc.axis);
                b.push_bind(&loc.nlcdesc16);
            });

            builder
                .push(" ON CONFLICT(nlc) DO UPDATE SET ")
                .push("stanox=excluded.stanox, ")
                .push("tiploc=excluded.tiploc, ")
                .push("crs=excluded.crs, ")
                .push("uic=excluded.uic, ")
                .push("nlcdesc=excluded.nlcdesc, ")
                .push("axis=excluded.axis, ")
                .push("nlcdesc16=excluded.nlcdesc16");

            let result = builder.build().execute(pool).await?;
            debug!("Bulk upsert affected {} rows", result.rows_affected());
        }
        Ok(())
    }
}
