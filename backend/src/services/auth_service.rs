use crate::auth::password;
use crate::domain::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::error::{AppError, AppResult};
use crate::repositories::user_repo;
use crate::state::AppState;

pub async fn register(state: &AppState, req: RegisterRequest) -> AppResult<AuthResponse> {
    req.validate()?;
    let hash = password::hash_password(&req.password)?;
    let user = user_repo::create(&state.pool, &req.email, &hash).await?;
    let token = state.jwt.issue(user.id, &user.email)?;
    Ok(AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    })
}

pub async fn login(state: &AppState, req: LoginRequest) -> AppResult<AuthResponse> {
    let user = user_repo::find_by_email(&state.pool, &req.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify_password(&req.password, &user.password_hash) {
        return Err(AppError::Unauthorized);
    }

    let token = state.jwt.issue(user.id, &user.email)?;
    Ok(AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    })
}
