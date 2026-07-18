use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::User;
use crate::error::{AppError, AppResult};

pub async fn create(pool: &PgPool, email: &str, password_hash: &str) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(email.to_lowercase())
    .bind(password_hash)
    .bind(Utc::now())
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db) if db.is_unique_violation() => {
            AppError::Conflict("an account with that email already exists".into())
        }
        other => AppError::Database(other),
    })?;
    Ok(user)
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email.to_lowercase())
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

#[allow(dead_code)] // part of the repository surface; not yet routed
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}
