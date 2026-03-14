use log::debug;
use std::collections::HashMap;

pub use crate::db::schema::Location;
use crate::providers::corpus::LocationEntry;

/// Determines the display value and type for a location based on priority.
/// Priority: nlcdesc -> nlcdesc16 -> crs -> stanox -> tiploc -> axis -> uic -> nlc
fn get_display_value(loc: &LocationEntry) -> (String, &'static str) {
    if let Some(ref v) = loc.nlcdesc {
        if !v.is_empty() {
            return (v.clone(), "nlcdesc");
        }
    }
    if let Some(ref v) = loc.nlcdesc16 {
        if !v.is_empty() {
            return (v.clone(), "nlcdesc16");
        }
    }
    if let Some(ref v) = loc.crs {
        if !v.is_empty() {
            return (v.clone(), "crs");
        }
    }
    if let Some(ref v) = loc.stanox {
        if !v.is_empty() {
            return (v.clone(), "stanox");
        }
    }
    if let Some(ref v) = loc.tiploc {
        if !v.is_empty() {
            return (v.clone(), "tiploc");
        }
    }
    if let Some(ref v) = loc.axis {
        if !v.is_empty() {
            return (v.clone(), "axis");
        }
    }
    if let Some(ref v) = loc.uic {
        if !v.is_empty() {
            return (v.clone(), "uic");
        }
    }
    (loc.nlc.to_string(), "nlc")
}

impl Location {
    pub async fn _insert(pool: &sqlx::postgres::PgPool, loc: Location) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as::<_, Location>(
            "INSERT INTO locations (
                nlc,
                stanox,
                tiploc,
                crs,
                uic,
                nlcdesc,
                axis,
                nlcdesc16,
                display,
                display_type
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, nlc, stanox, tiploc, crs, uic, nlcdesc, axis, nlcdesc16, display, display_type",
        )
        .bind(loc.nlc)
        .bind(loc.stanox)
        .bind(loc.tiploc)
        .bind(loc.crs)
        .bind(loc.uic)
        .bind(loc.nlcdesc)
        .bind(loc.axis)
        .bind(loc.nlcdesc16)
        .bind(loc.display)
        .bind(loc.display_type)
        .fetch_one(pool)
        .await?;
        debug!("Inserted Allocation with ID: {:?}", row.id);
        Ok(())
    }

    pub async fn insert_bulk(
        pool: &sqlx::postgres::PgPool,
        locs: &[LocationEntry],
    ) -> Result<(), sqlx::Error> {
        if locs.is_empty() {
            return Ok(());
        }

        // Deduplicate by nlc - keep the last occurrence
        let mut deduped: HashMap<i64, &LocationEntry> = HashMap::new();
        for loc in locs {
            deduped.insert(loc.nlc, loc);
        }
        let unique_locs: Vec<&LocationEntry> = deduped.into_values().collect();

        for chunk in unique_locs.chunks(1000) {
            let mut builder = sqlx::QueryBuilder::new(
                "INSERT INTO locations (nlc, stanox, tiploc, crs, uic, nlcdesc, axis, nlcdesc16, display, display_type) ",
            );

            // Decide on preferred unit

            builder.push_values(chunk, |mut b, loc| {
                let (display, display_type) = get_display_value(loc);
                b.push_bind(loc.nlc.to_string());
                b.push_bind(&loc.stanox);
                b.push_bind(&loc.tiploc);
                b.push_bind(&loc.crs);
                b.push_bind(&loc.uic);
                b.push_bind(&loc.nlcdesc);
                b.push_bind(&loc.axis);
                b.push_bind(&loc.nlcdesc16);
                b.push_bind(display);
                b.push_bind(display_type);
            });

            builder
                .push(" ON CONFLICT(nlc) DO UPDATE SET ")
                .push("stanox=excluded.stanox, ")
                .push("tiploc=excluded.tiploc, ")
                .push("crs=excluded.crs, ")
                .push("uic=excluded.uic, ")
                .push("nlcdesc=excluded.nlcdesc, ")
                .push("axis=excluded.axis, ")
                .push("nlcdesc16=excluded.nlcdesc16, ")
                .push("display=excluded.display, ")
                .push("display_type=excluded.display_type");

            let result = builder.build().execute(pool).await?;
            debug!("Bulk upsert affected {} rows", result.rows_affected());
        }
        Ok(())
    }

    pub async fn get_locations_where_has_tiploc(
        pool: &sqlx::postgres::PgPool,
    ) -> Result<Vec<Location>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Location>(
            "SELECT * FROM locations
            WHERE
                tiploc != ' ';",
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }
}
