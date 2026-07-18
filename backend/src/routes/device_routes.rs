use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

use crate::auth::AuthUser;
use crate::domain::dto::RegisterDeviceRequest;
use crate::domain::models::DeviceToken;
use crate::error::AppResult;
use crate::repositories::device_repo;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/devices", post(register_device))
}

/// Register (or refresh) an Expo push token for the authenticated user.
async fn register_device(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<RegisterDeviceRequest>,
) -> AppResult<(StatusCode, Json<DeviceToken>)> {
    let device =
        device_repo::upsert(&state.pool, user.user_id, &req.token, req.platform).await?;
    Ok((StatusCode::CREATED, Json(device)))
}
