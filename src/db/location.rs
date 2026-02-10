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
}
