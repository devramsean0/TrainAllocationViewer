use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Allocation {
    pub id: Option<i64>,
    pub origin_datetime: String,
    pub origin_location: String,
    pub date: Option<String>,
    pub dest_location: String,
    pub dest_datetime: String,
    pub allocation_origin_datetime: String,
    pub allocation_origin_location: String,
    pub allocation_dest_datetime: String,
    pub allocation_dest_location: String,
}

#[derive(Debug, FromRow)]
pub struct Vehicle {
    pub id: Option<i64>,
    pub livery: String,
    pub decor: String,
    pub vehicle_type: String,
    pub specific_type: String,
}
