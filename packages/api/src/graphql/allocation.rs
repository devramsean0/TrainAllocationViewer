use async_graphql::{ComplexObject, Context};
use sqlx::SqlitePool;

pub use crate::db::schema::Allocation;
use crate::db::schema::{Location, ResourceGroup};

#[ComplexObject]
impl Allocation {
    #[graphql(name = "originLocation")]
    async fn origin_location_detail(&self, ctx: &Context<'_>) -> Result<Option<Location>, String> {
        let db = ctx
            .data::<SqlitePool>()
            .map_err(|e| e.message.to_string())?;
        sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = $1")
            .bind(&self.origin_location)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[graphql(name = "destLocation")]
    async fn dest_location_detail(&self, ctx: &Context<'_>) -> Result<Option<Location>, String> {
        let db = ctx
            .data::<SqlitePool>()
            .map_err(|e| e.message.to_string())?;
        sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = $1")
            .bind(&self.dest_location)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[graphql(name = "allocationOriginLocation")]
    async fn allocation_origin_location_detail(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<Location>, String> {
        let db = ctx
            .data::<SqlitePool>()
            .map_err(|e| e.message.to_string())?;
        sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = $1")
            .bind(&self.allocation_origin_location)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())
    }

    #[graphql(name = "allocationDestLocation")]
    async fn allocation_dest_location_detail(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<Location>, String> {
        let db = ctx
            .data::<SqlitePool>()
            .map_err(|e| e.message.to_string())?;
        sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = $1")
            .bind(&self.allocation_dest_location)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())
    }

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
