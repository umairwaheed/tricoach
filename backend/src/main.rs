mod ai;
mod auth;
mod config;
mod db;
mod domain;
mod error;
mod notifications;
mod repositories;
mod routes;
mod services;
mod state;
mod telemetry;

use std::sync::Arc;

use ai::gemini::GeminiCoach;
use ai::rule_based::RuleBasedCoach;
use ai::AiCoach;
use auth::JwtEncoder;
use config::Config;
use notifications::PushSender;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init();

    let config = Config::from_env()?;
    tracing::info!(bind = %config.bind_addr, "starting TriCoach API");

    let pool = db::connect(&config.database_url).await?;
    db::migrate(&pool).await?;
    tracing::info!("migrations applied");

    // Choose the AI coach based on whether a Gemini key is configured.
    let coach: Arc<dyn AiCoach> = match &config.gemini_api_key {
        Some(key) => {
            tracing::info!(model = %config.gemini_model, "using Gemini-backed coach");
            Arc::new(GeminiCoach::new(key.clone(), config.gemini_model.clone()))
        }
        None => {
            tracing::info!("no GEMINI_API_KEY set — using deterministic rule-based coach");
            Arc::new(RuleBasedCoach)
        }
    };

    // Push is enabled whenever a Gemini key is present is unrelated — here we
    // simply enable real sending outside of tests; tokens still must be registered.
    let push = Arc::new(PushSender::new(true));

    let state = AppState {
        pool,
        jwt: JwtEncoder::new(config.jwt_secret.clone(), config.jwt_ttl_hours),
        coach,
        push,
    };

    let app = routes::app_router(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("listening on {}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
