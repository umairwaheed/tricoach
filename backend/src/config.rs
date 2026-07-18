use std::env;

/// Runtime configuration loaded from environment variables.
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub jwt_secret: String,
    pub jwt_ttl_hours: i64,
    /// When set, the Gemini-backed coach is used; otherwise the deterministic engine.
    pub gemini_api_key: Option<String>,
    pub gemini_model: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        // Load .env if present; ignore if it is not.
        let _ = dotenvy::dotenv();

        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://tricoach:tricoach@localhost:5432/tricoach".into()),
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-insecure-change-me".into()),
            jwt_ttl_hours: env::var("JWT_TTL_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(720),
            gemini_api_key: env::var("GEMINI_API_KEY").ok().filter(|s| !s.is_empty()),
            gemini_model: env::var("GEMINI_MODEL")
                .unwrap_or_else(|_| "gemini-2.0-flash".into()),
        })
    }
}
