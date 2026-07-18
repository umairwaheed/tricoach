use uuid::Uuid;

use crate::ai::{FeedbackContext, WorkoutContext};
use crate::domain::dto::SubmitFeedbackRequest;
use crate::domain::enums::WorkoutStatus;
use crate::domain::models::{Workout, WorkoutFeedback};
use crate::error::{AppError, AppResult};
use crate::repositories::{feedback_repo, profile_repo, workout_repo};
use crate::state::AppState;

/// Record what the athlete actually did, generate AI feedback, and mark the
/// workout completed — all guarded by ownership.
pub async fn submit_feedback(
    state: &AppState,
    user_id: Uuid,
    workout_id: Uuid,
    req: SubmitFeedbackRequest,
) -> AppResult<WorkoutFeedback> {
    req.validate()?;

    let workout = workout_repo::get_owned(&state.pool, workout_id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("workout".into()))?;

    let profile = profile_repo::get(&state.pool, user_id).await?;

    let workout_ctx = WorkoutContext {
        discipline: workout.discipline,
        title: workout.title.clone(),
        intensity: workout.intensity,
        planned_duration_min: workout.planned_duration_min,
        planned_distance_km: workout.planned_distance_km,
    };
    let feedback_ctx = FeedbackContext {
        actual_duration_min: req.actual_duration_min,
        actual_distance_km: req.actual_distance_km,
        avg_hr: req.avg_hr,
        max_hr: req.max_hr,
        perceived_effort: req.perceived_effort,
        notes: req.notes.clone(),
        athlete_max_hr: profile.as_ref().and_then(|p| p.max_hr),
    };

    let ai_feedback = state
        .coach
        .analyze_workout(&workout_ctx, &feedback_ctx)
        .await?;

    workout_repo::update_status(&state.pool, workout_id, WorkoutStatus::Completed).await?;
    let saved = feedback_repo::upsert(&state.pool, workout_id, &req, &ai_feedback).await?;
    Ok(saved)
}

pub async fn set_status(
    state: &AppState,
    user_id: Uuid,
    workout_id: Uuid,
    status: WorkoutStatus,
) -> AppResult<Workout> {
    // Ensure ownership before mutating.
    workout_repo::get_owned(&state.pool, workout_id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("workout".into()))?;
    workout_repo::update_status(&state.pool, workout_id, status).await
}
