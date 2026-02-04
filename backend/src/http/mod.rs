use axum::http::StatusCode;
use crate::models::ApiResponse;
mod health;
mod quadlets;
mod users;

pub use health::router as health_router;
pub use quadlets::router as quadlets_router;
pub use users::router as users_router;

pub async fn fallback_404() -> impl axum::response::IntoResponse {
    ApiResponse::new(
        StatusCode::NOT_FOUND,
        "Not found",
        None,
    )
}
