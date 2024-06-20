use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{get, post, response::content::RawHtml, routes, Route, State};

pub type GraphqlSchema<Query, Mutation = EmptyMutation, Subscription = EmptySubscription> =
    State<Schema<Query, Mutation, Subscription>>;

#[get("/")]
pub fn graphiql() -> RawHtml<String> {
    RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[get("/?<query..>")]
async fn graphql_query(schema: &GraphqlSchema, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[post("/", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &GraphqlSchema, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

pub fn graphql() -> impl Into<Vec<Route>> {
    routes![graphiql, graphql_query, graphql_request]
}
