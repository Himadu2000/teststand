mod cors;
mod graphql;

pub use async_graphql::{self, Object, Schema};
pub use async_graphql_rocket;
pub use cors::Cors;
pub use graphql::{graphiql, GQLSchema};
pub use reqwest;
pub use rocket;
use rocket::{get, response::Redirect, uri};
pub use serde::{Deserialize, Serialize};
pub use surrealdb::{self, sql::Thing};

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/graphql"))
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
