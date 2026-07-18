use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::domain::dto::{GeneratePlanRequest, PlanWithWorkouts};
use crate::domain::models::TrainingPlan;
use crate::error::AppResult;
use crate::repositories::plan_repo;
use crate::services::plan_service;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/plans", post(generate_plan).get(list_plans))
        .route("/plans/active", get(active_plan))
        .route("/plans/:id", get(get_plan))
}

async fn generate_plan(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<GeneratePlanRequest>,
) -> AppResult<(StatusCode, Json<PlanWithWorkouts>)> {
    let plan = plan_service::generate(&state, user.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(plan)))
}

async fn list_plans(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<Vec<TrainingPlan>>> {
    let plans = plan_repo::list_for_user(&state.pool, user.user_id).await?;
    Ok(Json(plans))
}

async fn active_plan(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<PlanWithWorkouts>> {
    let plan = plan_service::get_active_plan(&state, user.user_id).await?;
    Ok(Json(plan))
}

async fn get_plan(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<PlanWithWorkouts>> {
    let plan = plan_service::get_plan_with_workouts(&state, user.user_id, id).await?;
    Ok(Json(plan))
}
