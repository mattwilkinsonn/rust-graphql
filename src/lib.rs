use async_graphql::Context;
use sqlx::{Pool, Postgres};

pub fn get_pool_from_ctx<'a>(ctx: &Context<'a>) -> &'a Pool<Postgres> {
    return ctx.data::<Pool<Postgres>>().expect("Can't get pool");
}
