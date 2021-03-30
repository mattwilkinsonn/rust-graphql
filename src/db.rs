use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_db_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("Database connection failed");

    return db_pool;
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1;", id)
        .fetch_one(pool)
        .await?;

    return Ok(user);
}
