use log::debug;
use std::collections::HashMap;

pub use crate::db::schema::ReferenceCode;

impl ReferenceCode {
    pub async fn _insert(
        pool: &sqlx::postgres::PgPool,
        code: ReferenceCode,
    ) -> Result<(), sqlx::Error> {
        let row = sqlx::query_as::<_, ReferenceCode>(
            "INSERT INTO reference_codes (
                action_code,
                code_type,
                code,
                description
            ) VALUES ($1, $2, $3, $4)
            RETURNING id, action_code, code_type, code, description;",
        )
        .bind(code.action_code)
        .bind(code.code_type)
        .bind(code.code)
        .bind(code.description)
        .fetch_one(pool)
        .await?;
        debug!("Inserted Reference Code with ID: {:?}", row.id);
        Ok(())
    }

    pub async fn insert_bulk(
        pool: &sqlx::postgres::PgPool,
        codes: &[ReferenceCode],
    ) -> Result<(), sqlx::Error> {
        if codes.is_empty() {
            return Ok(());
        }

        // Deduplicate by nlc - keep the last occurrence
        let mut deduped: HashMap<String, &ReferenceCode> = HashMap::new();
        for code in codes {
            deduped.insert(code.description.clone(), code);
        }
        let unique_codes: Vec<&ReferenceCode> = deduped.into_values().collect();

        for chunk in unique_codes.chunks(1000) {
            let mut builder = sqlx::QueryBuilder::new(
                "INSERT INTO reference_codes (action_code, code_type, code, description) ",
            );

            builder.push_values(chunk, |mut b, code| {
                b.push_bind(&code.action_code);
                b.push_bind(&code.code_type);
                b.push_bind(&code.code);
                b.push_bind(&code.description);
            });

            builder
                .push(" ON CONFLICT(description) DO UPDATE SET ")
                .push("action_code=excluded.action_code, ")
                .push("code_type=excluded.code_type, ")
                .push("code=excluded.code, ")
                .push("description=excluded.description ");

            let result = builder.build().execute(pool).await?;
            debug!("Bulk upsert affected {} rows", result.rows_affected());
        }
        Ok(())
    }
}
