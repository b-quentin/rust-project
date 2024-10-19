use std::error::Error as Error;

pub trait CustomGraphQLError: Error + Send + Sync {
    fn new(&self) -> async_graphql::Error;
}
