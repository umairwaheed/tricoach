use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

use crate::domain::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::error::AppResult;
use crate::services::auth_service;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<(StatusCode, Json<AuthResponse>)> {
    let resp = auth_service::register(&state, req).await?;
    Ok((StatusCode::CREATED, Json(resp)))
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let resp = auth_service::login(&state, req).await?;
    Ok(Json(resp))
}
