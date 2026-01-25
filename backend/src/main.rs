mod http;
mod models;

use axum::Router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Configurar CORS para desarrollo
    let cors = CorsLayer::permissive();

    // Crear el router principal
    let app = Router::new()
        .merge(http::create_router())
        .layer(cors);

    // Iniciar el servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 QuMa server listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
