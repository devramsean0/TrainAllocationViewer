use async_graphql::{ComplexObject, Context};
use sqlx::PgPool;

use crate::db::schema::ResourceGroup;
pub use crate::db::schema::Vehicle;

#[ComplexObject]
impl Vehicle {
    #[graphql(name = "resourceGroup")]
    async fn resource_group_detail(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<ResourceGroup>, String> {
        let db = ctx.data::<PgPool>().map_err(|e| e.message.to_string())?;
        sqlx::query_as::<_, ResourceGroup>("SELECT * FROM resource_groups WHERE id = $1")
            .bind(&self.resource_group_id)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[graphql(name = "specialCharacteristics")]
    async fn special_characteristics_detail(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<String>, String> {
        let db = ctx.data::<PgPool>().map_err(|e| e.message.to_string())?;
        let result = sqlx::query_scalar::<_, String>(
            "SELECT description FROM reference_codes WHERE code_type = 'OperatingCharacteristic' AND code = $1"
        )
            .bind(&self.special_characteristics)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())?;
        let value = result.or(self.special_characteristics.clone());

        if value.is_some() {
            let value = value
                .unwrap()
                .split(" ")
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>()
                .join(" ");
            return Ok(Some(value));
        } else {
            return Ok(value);
        }
    }
}
