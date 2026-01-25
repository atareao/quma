pub mod quadlets;
pub mod users;

use axum::Router;

/// Crea el router principal de la API
pub fn create_router() -> Router {
    Router::new()
        .nest("/api/quadlets", quadlets::router())
        .nest("/api/users", users::router())
}
