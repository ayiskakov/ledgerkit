use axum::{routing::get, Router};

use crate::health;

/// Create the axum router with all routes.
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/ready", get(health::ready_check))
}
