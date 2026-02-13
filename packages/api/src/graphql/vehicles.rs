use async_graphql::{ComplexObject, Context};
use sqlx::SqlitePool;

use crate::db::schema::ResourceGroup;
pub use crate::db::schema::Vehicle;

#[ComplexObject]
impl Vehicle {
    #[graphql(name = "resourceGroup")]
    async fn resource_group_detail(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<ResourceGroup>, String> {
        let db = ctx
            .data::<SqlitePool>()
            .map_err(|e| e.message.to_string())?;
        sqlx::query_as::<_, ResourceGroup>("SELECT * FROM resource_groups WHERE id = $1")
            .bind(&self.resource_group_id)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())
    }
}
