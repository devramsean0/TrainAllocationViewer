use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Allocation {
    pub id: Option<i64>,
    pub origin_datetime: String,
    #[graphql(skip)]
    pub origin_location: String,
    pub origin_country_code_iso: Option<String>,
    pub origin_subsidiary_information_code: Option<String>,
    pub origin_subsidiary_information_company: Option<String>,
    pub date: Option<String>,
    #[graphql(skip)]
    pub dest_location: String,
    pub dest_country_code_iso: Option<String>,
    pub dest_subsidiary_information_code: Option<String>,
    pub dest_subsidiary_information_company: Option<String>,
    pub dest_datetime: String,
    pub allocation_origin_datetime: String,
    #[graphql(skip)]
    pub allocation_origin_location: String,
    pub allocation_origin_country_code_iso: Option<String>,
    pub allocation_origin_subsidiary_information_code: Option<String>,
    pub allocation_origin_subsidiary_information_company: Option<String>,
    pub allocation_dest_datetime: String,
    #[graphql(skip)]
    pub allocation_dest_location: String,
    pub allocation_dest_country_code_iso: Option<String>,
    pub allocation_dest_subsidiary_information_code: Option<String>,
    pub allocation_dest_subsidiary_information_company: Option<String>,
    #[graphql(skip)]
    pub resource_group_id: String,
    pub sequence_number: Option<i64>,
    pub resource_group_position: Option<i64>,
    pub diagram_no: Option<String>,
    pub origin_miles: Option<i64>,
    pub destination_miles: Option<i64>,
    pub reversed: Option<String>,
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
    pub resource_position: Option<i64>,
    pub planned_resource_group: Option<String>,
    pub length_value: Option<String>,
    pub length_measure: Option<String>,
    pub weight: Option<i32>,
    #[graphql(skip)]
    pub special_characteristics: Option<String>,
    pub seat_count: Option<i32>,
    pub cab_count: Option<i32>,
    pub date_entered_service: Option<String>,
    pub date_registered: Option<String>,
    pub category: Option<String>,
    pub brake_type: Option<String>,
    pub max_speed: Option<String>,
}

#[derive(Debug, Clone, FromRow, SimpleObject)]
pub struct ResourceGroup {
    pub id: String,
    pub fleet: String,
    pub resource_type: Option<String>,
    pub status: Option<String>,
    pub end_of_day_miles: Option<String>,
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
    pub display: String,
    pub display_type: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct AllocArchiveLog {
    pub id: Option<i64>,
    pub file_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct BplanLog {
    pub id: Option<i64>,
    pub file_version: String,
    pub source_system: String,
    pub toc_id: String,
    pub timetable_start_date: String,
    pub timetable_end_date: String,
    pub cycle_type: String,
    pub cycle_stage: String,
    pub creation_date: String,
    pub sequence_number: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct ReferenceCode {
    pub id: Option<i64>,
    pub action_code: String,
    pub code_type: String,
    pub code: Option<String>,
    pub description: String,
}
