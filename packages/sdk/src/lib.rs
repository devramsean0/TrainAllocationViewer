uniffi::setup_scaffolding!();

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/allocations/forfleet.graphql"
)]
pub struct AllocationsForFleetQuery;

#[uniffi::export]
pub async fn get_allocations_for_fleet(fleet: String) {
    let variables = allocations_for_fleet_query::Variables { fleet };
    let body = AllocationsForFleetQuery::build_query(variables);

    let client = reqwest::blocking::Client::new();
}
