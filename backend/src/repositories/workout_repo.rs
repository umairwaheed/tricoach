use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::enums::WorkoutStatus;
use crate::domain::models::Workout;
use crate::error::AppResult;

pub async fn list_for_plan(pool: &PgPool, plan_id: Uuid) -> AppResult<Vec<Workout>> {
    let workouts = sqlx::query_as::<_, Workout>(
        "SELECT * FROM workouts WHERE plan_id = $1 ORDER BY scheduled_date, discipline",
    )
    .bind(plan_id)
    .fetch_all(pool)
    .await?;
    Ok(workouts)
}

/// Fetch a workout only if it belongs to the given user (ownership folded into
/// the join). Returns `None` both for missing and for not-owned workouts.
pub async fn get_owned(
    pool: &PgPool,
    workout_id: Uuid,
    user_id: Uuid,
) -> AppResult<Option<Workout>> {
    let workout = sqlx::query_as::<_, Workout>(
        r#"
        SELECT w.*
        FROM workouts w
        JOIN training_plans p ON p.id = w.plan_id
        WHERE w.id = $1 AND p.user_id = $2
        "#,
    )
    .bind(workout_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(workout)
}

pub async fn update_status(
    pool: &PgPool,
    workout_id: Uuid,
    status: WorkoutStatus,
) -> AppResult<Workout> {
    let workout = sqlx::query_as::<_, Workout>(
        "UPDATE workouts SET status = $1 WHERE id = $2 RETURNING *",
    )
    .bind(status)
    .bind(workout_id)
    .fetch_one(pool)
    .await?;
    Ok(workout)
}

/// Workouts for a user within a date window (used by the schedule view).
pub async fn list_for_user_between(
    pool: &PgPool,
    user_id: Uuid,
    from: NaiveDate,
    to: NaiveDate,
) -> AppResult<Vec<Workout>> {
    let workouts = sqlx::query_as::<_, Workout>(
        r#"
        SELECT w.*
        FROM workouts w
        JOIN training_plans p ON p.id = w.plan_id
        WHERE p.user_id = $1 AND p.status = 'active'
          AND w.scheduled_date BETWEEN $2 AND $3
        ORDER BY w.scheduled_date, w.discipline
        "#,
    )
    .bind(user_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    Ok(workouts)
}
