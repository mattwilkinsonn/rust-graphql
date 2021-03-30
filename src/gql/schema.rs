use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::{Pool, Postgres};

use super::query::QueryRoot;

pub type GQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema(pool: Pool<Postgres>) -> GQLSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish()
}
