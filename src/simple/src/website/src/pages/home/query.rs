#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct UnnamedQuery {
    pub status: String,
}
