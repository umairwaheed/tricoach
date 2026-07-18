use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

/// JWT claims. `sub` carries the user id.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Clone)]
pub struct JwtEncoder {
    secret: String,
    ttl_hours: i64,
}

impl JwtEncoder {
    pub fn new(secret: String, ttl_hours: i64) -> Self {
        Self { secret, ttl_hours }
    }

    pub fn issue(&self, user_id: Uuid, email: &str) -> AppResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(self.ttl_hours)).timestamp(),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Other(anyhow::anyhow!("failed to issue token: {e}")))
    }

    pub fn verify(&self, token: &str) -> AppResult<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
    }
}
