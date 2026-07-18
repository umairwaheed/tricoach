use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::SqlitePool;

use crate::ai::AiCoach;
use crate::auth::JwtEncoder;
use crate::notifications::PushSender;

/// Shared application state, cloned cheaply into every handler.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub jwt: JwtEncoder,
    pub coach: Arc<dyn AiCoach>,
    pub push: Arc<PushSender>,
}

// Allows `AuthUser` (and anything else) to extract just the encoder from state.
impl FromRef<AppState> for JwtEncoder {
    fn from_ref(state: &AppState) -> Self {
        state.jwt.clone()
    }
}
