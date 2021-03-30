use async_graphql::Context;
use sqlx::{Pool, Postgres};

use crate::server::MyToken;

pub fn get_pool_from_ctx<'a>(ctx: &Context<'a>) -> &'a Pool<Postgres> {
    return ctx.data::<Pool<Postgres>>().expect("Can't get pool");
}

pub fn get_token_from_ctx<'a>(ctx: &Context<'a>) -> &'a MyToken {
    return ctx.data::<MyToken>().expect("Can't get token");
}
