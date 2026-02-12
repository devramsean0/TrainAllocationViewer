use async_graphql::{
    context::Context, http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use log::info;
use sqlx::SqlitePool;
use tokio::{spawn, sync::broadcast::Sender};

pub mod allocation;
pub mod vehicles;

use crate::db::schema::{Allocation, Location, ResourceGroup, Vehicle};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &'static str {
        "world"
    }

    async fn locations(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Filter by NLC")] nlc: Option<String>,
    ) -> Result<Option<Vec<Location>>, String> {
        let db = match ctx.data::<SqlitePool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE (CASE WHEN $1 IS NOT NULL THEN (nlc = $1) ELSE (id = id) END);").bind(nlc)
            .fetch_all(db)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        Ok(Some(res))
    }

    async fn resouce_groups(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Filter by ID")] id: Option<String>,
        #[graphql(desc = "Filter by SpecificType")] specific_type: Option<String>,
    ) -> Result<Option<Vec<ResourceGroup>>, String> {
        let db = match ctx.data::<SqlitePool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, ResourceGroup>(
            "
            SELECT * FROM resource_groups
                WHERE (CASE WHEN $1 IS NOT NULL THEN (id = $1) ELSE (id = id) END)
                AND (CASE WHEN $2 IS NOT NULL THEN (fleet = $2) ELSE (fleet = fleet) END)
            ;",
        )
        .bind(id)
        .bind(specific_type)
        .fetch_all(db)
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Some(res))
    }

    async fn vehicles(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Filter by ID")] id: Option<i64>,
        #[graphql(desc = "Filter by Livery")] livery: Option<String>,
        #[graphql(desc = "Filter by Decor")] decor: Option<String>,
        #[graphql(desc = "Filter by Vehicle Type")] vehicle_type: Option<String>,
        #[graphql(desc = "Filter by Specific Type")] specific_type: Option<String>,
        #[graphql(desc = "Filter by Resource Group ID")] resource_group_id: Option<String>,
    ) -> Result<Option<Vec<Vehicle>>, String> {
        let db = match ctx.data::<SqlitePool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Vehicle>(
            "
            SELECT * FROM vehicles
                WHERE (CASE WHEN $1 IS NOT NULL THEN (id = $1) ELSE (id = id) END)
                AND (CASE WHEN $2 IS NOT NULL THEN (livery = $2) ELSE (livery = livery) END)
                AND (CASE WHEN $3 IS NOT NULL THEN (decor = $3) ELSE (decor = decor) END)
                AND (CASE WHEN $4 IS NOT NULL THEN (vehicle_type = $4) ELSE (vehicle_type = vehicle_type) END)
                AND (CASE WHEN $5 IS NOT NULL THEN (specific_type = $5) ELSE (specific_type = specific_type) END)
                AND (CASE WHEN $6 IS NOT NULL THEN (resource_group_id = $6) ELSE (resource_group_id = resource_group_id) END)
            ;",
        )
        .bind(id)
        .bind(livery)
        .bind(decor)
        .bind(vehicle_type)
        .bind(specific_type)
        .bind(resource_group_id)
        .fetch_all(db)
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Some(res))
    }

    async fn allocations(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Filter by ID")] id: Option<i64>,
        #[graphql(desc = "Filter by Date")] date: Option<String>,
        #[graphql(desc = "Filter by Origin Location")] origin_location: Option<String>,
        #[graphql(desc = "Filter by Destination Location")] dest_location: Option<String>,
        #[graphql(desc = "Filter by Allocation Origin Location")]
        allocation_origin_location: Option<String>,
        #[graphql(desc = "Filter by Allocation Destination Location")]
        allocation_dest_location: Option<String>,
        #[graphql(desc = "Filter by Resource Group ID")] resource_group_id: Option<String>,
        #[graphql(desc = "Filter by Fleet")] fleet: Option<String>,
    ) -> Result<Option<Vec<Allocation>>, String> {
        let db = match ctx.data::<SqlitePool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, Allocation>(
            "
            SELECT allocations.* FROM allocations
                LEFT JOIN resource_groups ON allocations.resource_group_id = resource_groups.id
                WHERE (CASE WHEN $1 IS NOT NULL THEN (allocations.id = $1) ELSE (allocations.id = allocations.id) END)
                AND (CASE WHEN $2 IS NOT NULL THEN (allocations.date = $2) ELSE (allocations.date = allocations.date) END)
                AND (CASE WHEN $3 IS NOT NULL THEN (allocations.origin_location = $3) ELSE (allocations.origin_location = allocations.origin_location) END)
                AND (CASE WHEN $4 IS NOT NULL THEN (allocations.dest_location = $4) ELSE (allocations.dest_location = allocations.dest_location) END)
                AND (CASE WHEN $5 IS NOT NULL THEN (allocations.allocation_origin_location = $5) ELSE (allocations.allocation_origin_location = allocations.allocation_origin_location) END)
                AND (CASE WHEN $6 IS NOT NULL THEN (allocations.allocation_dest_location = $6) ELSE (allocations.allocation_dest_location = allocations.allocation_dest_location) END)
                AND (CASE WHEN $7 IS NOT NULL THEN (allocations.resource_group_id = $7) ELSE (allocations.resource_group_id = allocations.resource_group_id) END)
                AND (CASE WHEN $8 IS NOT NULL THEN (resource_groups.fleet = $8) ELSE (resource_groups.fleet = resource_groups.fleet OR resource_groups.fleet IS NULL) END)
            ;",
        )
        .bind(id)
        .bind(date)
        .bind(origin_location)
        .bind(dest_location)
        .bind(allocation_origin_location)
        .bind(allocation_dest_location)
        .bind(resource_group_id)
        .bind(fleet)
        .fetch_all(db)
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Some(res))
    }
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn serve(pool: &SqlitePool, sender: &Sender<()>) -> anyhow::Result<()> {
    let mut shutdown = sender.subscribe();

    let pool = pool.clone();
    spawn(async move {
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(pool)
            .finish();

        let router = Router::new().route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        );

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        let _ = axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                let _ = shutdown.recv().await;
                info!("GraphQL recieved Ctrl+C, Exiting")
            })
            .await;
    });
    Ok(())
}
