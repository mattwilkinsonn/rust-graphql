use super::super::gql::mutation::RegisterInput;
use chrono::{Date, DateTime, Utc};
use sqlx::{Pool, Postgres};

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1;", id)
        .fetch_one(pool)
        .await?;

    return Ok(user);
}

pub struct CreateUser {
    email: String,
    username: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<RegisterInput> for CreateUser {
    fn from(register_input: RegisterInput) -> Self {
        CreateUser {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            email: register_input.email,
            username: register_input.username,
            password: register_input.password,
        }
    }
}

pub async fn create_user(pool: &Pool<Postgres>, data: CreateUser) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users
    (username, email, password, created_at, updated_at) 
     VALUES ($1, $2, $3, $4, $5) 
     RETURNING *;",
        data.username,
        data.email,
        data.password,
        data.created_at,
        data.updated_at
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
