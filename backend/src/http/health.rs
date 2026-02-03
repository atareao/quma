use crate::models::{ApiResponse, AppState, Data};
use axum::{Router, http::StatusCode, response::IntoResponse, routing};
use std::sync::Arc;

pub fn health_router() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::get(check_health))
}

async fn check_health() -> impl IntoResponse {
    ApiResponse::create(StatusCode::OK, "Up and running", Data::None)
}
