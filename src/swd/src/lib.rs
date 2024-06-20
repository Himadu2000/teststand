mod cors;
mod graphql;

pub use async_graphql;
pub use async_graphql_rocket;
pub use cors::Cors;
pub use graphql::{graphiql, GQLSchema};
pub use reqwest;
pub use rocket;

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
