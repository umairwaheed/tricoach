use axum::extract::State;
use axum::routing::{get, put};
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::domain::dto::UpsertProfileRequest;
use crate::domain::models::AthleteProfile;
use crate::error::{AppError, AppResult};
use crate::repositories::profile_repo;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/profile", put(upsert_profile))
        .route("/profile", get(get_profile))
}

async fn upsert_profile(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<UpsertProfileRequest>,
) -> AppResult<Json<AthleteProfile>> {
    req.validate()?;
    let profile = profile_repo::upsert(&state.pool, user.user_id, &req).await?;
    Ok(Json(profile))
}

async fn get_profile(
    State(state): State<AppState>,
    user: AuthUser,
) -> AppResult<Json<AthleteProfile>> {
    let profile = profile_repo::get(&state.pool, user.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("profile".into()))?;
    Ok(Json(profile))
}
