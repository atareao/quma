mod http;
mod models;
mod constants;

use axum::Router;
use models::Error;
use tower_http::{
    services::{
        ServeDir,
        ServeFile
    },
    trace::TraceLayer,
    cors::{
        CorsLayer,
        Any,
    },
};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use tracing::{
    info,
    debug,
};

const STATIC_DIR: &str = "static";

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let log_level = var("RUST_LOG").unwrap_or("debug".to_string());
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Log level: {log_level}");
    let port = var("PORT").unwrap_or("3000".to_string());
    info!("Port: {}", port);
    let secret = var("SECRET").unwrap_or("esto-es-un-secreto".to_string());
    debug!("Secret: {}", secret);

    // Configurar CORS para desarrollo
    let cors = CorsLayer::permissive();
    let api_routes = Router::new()
        .nest("/quadlets", http::quadlets_router())
        .nest("/users", http::users_router())
        .nest("/health", http::health_router())
        .fallback(http::fallback_404)
        .with_state(Arc::new(AppState {
            secret,
            static_dir: STATIC_DIR.to_string(),
    }));

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
