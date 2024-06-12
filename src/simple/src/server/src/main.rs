use async_graphql::{EmptyMutation, EmptySubscription, Schema};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    rocket::build()
        .attach(Cors)
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
