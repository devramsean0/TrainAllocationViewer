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

use crate::db::schema::Location;

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &'static str {
        "world"
    }

    async fn locations(&self, ctx: &Context<'_>) -> Result<Option<Vec<Location>>, String> {
        let db = match ctx.data::<SqlitePool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        // write an sql query to grab all the fields we need
        // change the SQL query accordingly if you don't need it
        let res = match sqlx::query_as::<_, Location>("SELECT * FROM locations;")
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
