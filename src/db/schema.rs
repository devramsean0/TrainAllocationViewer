use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Allocation {
    pub id: Option<i64>,
    pub origin_datetime: String,
    #[graphql(skip)]
    pub origin_location: String,
    pub date: Option<String>,
    #[graphql(skip)]
    pub dest_location: String,
    pub dest_datetime: String,
    pub allocation_origin_datetime: String,
    #[graphql(skip)]
    pub allocation_origin_location: String,
    pub allocation_dest_datetime: String,
    #[graphql(skip)]
    pub allocation_dest_location: String,
    #[graphql(skip)]
    pub resource_group_id: String,
}

#[derive(Debug, Clone, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Vehicle {
    pub id: Option<i64>,
    pub livery: String,
    pub decor: Option<String>,
    pub vehicle_type: String,
    pub specific_type: String,
    #[graphql(skip)]
    pub resource_group_id: String,
}

#[derive(Debug, Clone, FromRow, SimpleObject)]
pub struct ResourceGroup {
    pub id: String,
    pub fleet: String,
}

#[derive(Debug, Clone, FromRow, SimpleObject)]
pub struct Location {
    pub id: Option<i64>,
    pub nlc: String,
    pub stanox: Option<String>,
    pub tiploc: Option<String>,
    pub crs: Option<String>,
    pub uic: Option<String>,
    pub nlcdesc: Option<String>,
    pub axis: Option<String>,
    pub nlcdesc16: Option<String>,
}
