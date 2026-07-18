use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{patch, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::domain::dto::{SubmitFeedbackRequest, UpdateWorkoutStatusRequest};
use crate::domain::models::{Workout, WorkoutFeedback};
use crate::error::{AppError, AppResult};
use crate::repositories::{feedback_repo, workout_repo};
use crate::services::workout_service;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/workouts/:id/feedback",
            post(submit_feedback).get(get_feedback),
        )
        .route("/workouts/:id/status", patch(update_status))
}

async fn submit_feedback(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<SubmitFeedbackRequest>,
) -> AppResult<(StatusCode, Json<WorkoutFeedback>)> {
    let feedback = workout_service::submit_feedback(&state, user.user_id, id, req).await?;
    Ok((StatusCode::CREATED, Json(feedback)))
}

async fn get_feedback(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<WorkoutFeedback>> {
    // Ownership check via the workout.
    workout_repo::get_owned(&state.pool, id, user.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("workout".into()))?;
    let feedback = feedback_repo::get_for_workout(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("feedback".into()))?;
    Ok(Json(feedback))
}

async fn update_status(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateWorkoutStatusRequest>,
) -> AppResult<Json<Workout>> {
    let workout = workout_service::set_status(&state, user.user_id, id, req.status).await?;
    Ok(Json(workout))
}
