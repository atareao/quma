mod http;
mod models;

use axum::Router;
use models::Error;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
const STATIC_DIR: &str = "static";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Configurar CORS para desarrollo
    let cors = CorsLayer::permissive();
    let api_routes = Router::new()
        .nest("/quadlets", http::quadlets_router())
        .nest("/users", http::users_router())
        .nest("/health", http::health_router());

    // Crear el router principal
    let app = Router::new()
        .nest("/api/v1", api_routes)
        .fallback_service(ServeDir::new(STATIC_DIR).fallback(ServeFile::new("static/index.html")))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Iniciar el servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 QuMa server listening on http://localhost:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
