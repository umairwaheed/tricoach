use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::dto::SubmitFeedbackRequest;
use crate::domain::models::WorkoutFeedback;
use crate::error::AppResult;

pub async fn upsert(
    pool: &PgPool,
    workout_id: Uuid,
    req: &SubmitFeedbackRequest,
    ai_feedback: &str,
) -> AppResult<WorkoutFeedback> {
    let feedback = sqlx::query_as::<_, WorkoutFeedback>(
        r#"
        INSERT INTO workout_feedback
            (id, workout_id, actual_duration_min, actual_distance_km, avg_hr, max_hr,
             perceived_effort, notes, ai_feedback, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (workout_id) DO UPDATE SET
            actual_duration_min = excluded.actual_duration_min,
            actual_distance_km = excluded.actual_distance_km,
            avg_hr = excluded.avg_hr,
            max_hr = excluded.max_hr,
            perceived_effort = excluded.perceived_effort,
            notes = excluded.notes,
            ai_feedback = excluded.ai_feedback
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(workout_id)
    .bind(req.actual_duration_min)
    .bind(req.actual_distance_km)
    .bind(req.avg_hr)
    .bind(req.max_hr)
    .bind(req.perceived_effort)
    .bind(&req.notes)
    .bind(ai_feedback)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    Ok(feedback)
}

pub async fn get_for_workout(
    pool: &PgPool,
    workout_id: Uuid,
) -> AppResult<Option<WorkoutFeedback>> {
    let feedback =
        sqlx::query_as::<_, WorkoutFeedback>("SELECT * FROM workout_feedback WHERE workout_id = $1")
            .bind(workout_id)
            .fetch_optional(pool)
            .await?;
    Ok(feedback)
}
