use crate::{
    services::{Mutation, Query},
    util::{cors::Cors, graphql::graphql},
};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();

    rocket::build()
        .attach(Cors)
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
