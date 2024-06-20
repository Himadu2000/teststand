use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use rocket::{get, response::content::RawHtml, State};

pub type GQLSchema<Query, Mutation = EmptyMutation, Subscription = EmptySubscription> =
    State<Schema<Query, Mutation, Subscription>>;

#[get("/")]
pub fn graphiql() -> RawHtml<String> {
    RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}
