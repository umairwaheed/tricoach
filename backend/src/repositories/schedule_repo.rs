use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::dto::CreateScheduleBlockRequest;
use crate::domain::models::ScheduleBlock;
use crate::error::AppResult;

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateScheduleBlockRequest,
) -> AppResult<ScheduleBlock> {
    let block = sqlx::query_as::<_, ScheduleBlock>(
        r#"
        INSERT INTO schedule_blocks (id, user_id, title, starts_at, ends_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&req.title)
    .bind(req.starts_at)
    .bind(req.ends_at)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    Ok(block)
}

pub async fn list_between(
    pool: &PgPool,
    user_id: Uuid,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> AppResult<Vec<ScheduleBlock>> {
    let blocks = sqlx::query_as::<_, ScheduleBlock>(
        r#"
        SELECT * FROM schedule_blocks
        WHERE user_id = $1 AND starts_at < $3 AND ends_at > $2
        ORDER BY starts_at
        "#,
    )
    .bind(user_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    Ok(blocks)
}

pub async fn delete(pool: &PgPool, user_id: Uuid, block_id: Uuid) -> AppResult<u64> {
    let result = sqlx::query("DELETE FROM schedule_blocks WHERE id = $1 AND user_id = $2")
        .bind(block_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
