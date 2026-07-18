use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::enums::Platform;
use crate::domain::models::DeviceToken;
use crate::error::AppResult;

pub async fn upsert(
    pool: &SqlitePool,
    user_id: Uuid,
    token: &str,
    platform: Platform,
) -> AppResult<DeviceToken> {
    let device = sqlx::query_as::<_, DeviceToken>(
        r#"
        INSERT INTO device_tokens (id, user_id, token, platform, created_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (user_id, token) DO UPDATE SET platform = excluded.platform
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(token)
    .bind(platform)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    Ok(device)
}

pub async fn list_for_user(pool: &SqlitePool, user_id: Uuid) -> AppResult<Vec<DeviceToken>> {
    let devices =
        sqlx::query_as::<_, DeviceToken>("SELECT * FROM device_tokens WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
    Ok(devices)
}
