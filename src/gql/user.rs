use super::super::db;
use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    #[graphql(skip)]
    created_at: DateTime<Utc>,
    #[graphql(skip)]
    updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl User {
    async fn created_at(&self) -> i64 {
        self.created_at.timestamp()
    }
    async fn updated_at(&self) -> i64 {
        self.updated_at.timestamp()
    }
}

impl From<db::User> for User {
    fn from(db_user: db::User) -> Self {
        User {
            id: db_user.id,
            email: db_user.email,
            username: db_user.username,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
        }
    }
}
