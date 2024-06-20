use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use rocket::{get, post, response::content::RawHtml, routes, Route, State};

pub type GraphqlSchema<Query, Mutation = EmptyMutation, Subscription = EmptySubscription> =
    State<Schema<Query, Mutation, Subscription>>;

#[get("/")]
pub fn graphiql() -> RawHtml<String> {
    RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}
