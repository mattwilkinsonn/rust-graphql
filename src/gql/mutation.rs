use async_graphql::{Context, Error, InputObject, Object};

use crate::{
    db::user::{create_user, CreateUser},
    utils::get_pool_from_ctx,
};

use super::user::User;

#[derive(InputObject)]
pub struct RegisterInput {
    pub email: String,
    pub username: String,
    pub password: String,
}

struct Mutation;

#[Object]
impl Mutation {
    async fn register<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Register Input")] data: RegisterInput,
    ) -> Result<User, Error> {
        let pool = get_pool_from_ctx(ctx);

        let user = create_user(pool, CreateUser::from(data)).await?;

        Ok(User::from(user))
    }
}
