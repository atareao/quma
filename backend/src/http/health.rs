use crate::models::{ApiResponse, AppState };
use axum::{Router, http::StatusCode, response::IntoResponse, routing};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::get(check_health))
}

async fn check_health() -> impl IntoResponse {
    ApiResponse::new(StatusCode::OK, "Up and running", None)
}
