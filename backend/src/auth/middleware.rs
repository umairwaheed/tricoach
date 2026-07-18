use axum::async_trait;
use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use uuid::Uuid;

use super::jwt::JwtEncoder;
use crate::error::AppError;

/// Authenticated principal extracted from a `Bearer` token.
///
/// Any handler that takes `AuthUser` as an argument is automatically protected:
/// requests without a valid token are rejected with 401 before the handler runs.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    #[allow(dead_code)] // available to handlers; not needed by all
    pub email: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    JwtEncoder: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .ok_or(AppError::Unauthorized)?;

        let jwt = JwtEncoder::from_ref(state);
        let claims = jwt.verify(token.trim())?;

        Ok(AuthUser {
            user_id: claims.sub,
            email: claims.email,
        })
    }
}
