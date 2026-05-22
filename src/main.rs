mod api;
mod config;
mod db;
mod domain;
mod error;
mod repository;
mod service;
mod utils;

use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

use config::Settings;
use db::connection::establish_pool;
use api::handlers::metadata_handler::AppState;
use utils::helpers::setup_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load settings from environment
    let settings = Settings::from_env()?;

    // Setup logging
    setup_logging(&settings.log_level)?;

    info!("=== LLM REST API Server Starting ===");
    info!("Environment: {}", settings.environment);
    info!("Server address: {}", settings.server_address());

    // Create and configure database connection pool on startup
    info!("Initializing database connection pool...");
    let db_pool = establish_pool(&settings)?;
    info!("✓ Connection pool created with 5 idle connections (min) and 20 max connections");

    // Create application state with connection pool and settings
    let app_state = Arc::new(AppState {
        db_pool: db_pool.clone(),
        settings: settings.clone(),
    });

    // Build router with routes and middleware
    // Inject connection pool and settings into shared state (State) for handlers
    let app = Router::new()
        .route("/health", get(api::handlers::health_handler::health_check_handler))
        .route("/models", get(api::handlers::model_handler::list_models_handler))
        .route("/api/metadata", get(api::handlers::metadata_handler::get_combobox_metadata))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    
    info!("✓ Connection pool and settings injected into application state");
    info!("✓ Endpoint /api/metadata available for combobox search");

    // Bind and serve
    let listener = tokio::net::TcpListener::bind(settings.server_address())
        .await?;

    info!("Server is running and ready to accept requests");

    axum::serve(listener, app).await?;

    Ok(())
}