use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::error::AppResult;
use crate::state::AppState;

pub async fn root() -> Json<Value> {
    Json(json!({
        "name": "TriCoach API",
        "description": "AI-powered triathlon coaching backend",
        "docs": "/health for liveness; see README for endpoints"
    }))
}

/// Liveness + DB readiness probe.
pub async fn health(State(state): State<AppState>) -> AppResult<Json<Value>> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(json!({ "status": "ok", "database": "up" })))
}
