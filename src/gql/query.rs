pub struct QueryRoot;

use crate::db::user::get_user_by_id;

use super::super::utils::get_pool_from_ctx;
use super::user::User;
use async_graphql::{Context, Error, Object};

#[Object]
impl QueryRoot {
    async fn user<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "User's ID")] id: i32,
    ) -> Result<User, Error> {
        let pool = get_pool_from_ctx(ctx);

        let user = get_user_by_id(pool, id).await?;

        return Ok(User::from(user));
    }
}
