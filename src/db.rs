use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub mod user;

pub async fn get_db_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("Database connection failed");

    return db_pool;
}
