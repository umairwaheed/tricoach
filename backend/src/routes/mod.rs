pub mod auth_routes;
pub mod device_routes;
pub mod health;
pub mod plan_routes;
pub mod profile_routes;
pub mod schedule_routes;
pub mod workout_routes;

use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::state::AppState;

/// Assemble the full application router.
pub fn app_router(state: AppState) -> Router {
    let api = Router::new()
        .merge(auth_routes::router())
        .merge(profile_routes::router())
        .merge(plan_routes::router())
        .merge(workout_routes::router())
        .merge(schedule_routes::router())
        .merge(device_routes::router());

    Router::new()
        .route("/", get(health::root))
        .route("/health", get(health::health))
        .nest("/api/v1", api)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}
