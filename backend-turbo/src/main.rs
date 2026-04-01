use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

mod handlers;
mod ollama_client;
mod turbo_processor;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/generate", post(handlers::generate_sync))
        .route("/api/models", get(handlers::list_models_handler))
        .route("/api/enhance", post(handlers::enhance_response))
        .route("/ws/generate", get(handlers::ws_generate))
        .layer(cors);

    let addr: SocketAddr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8001".to_string())
        .parse()?;

    tracing::info!("Turbo backend listening on {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
