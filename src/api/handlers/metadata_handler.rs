use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;
use crate::db::connection::OraclePool;
use crate::config::Settings;

/// Query parameter for metadata search endpoint
#[derive(Deserialize)]
pub struct SearchParams {
    pub search: Option<String>,
}

/// Response structure for metadata search
#[derive(Serialize)]
pub struct MetadataResponse {
    pub results: Vec<String>,
    pub total: usize,
}

/// State structure containing database pool and configuration
#[derive(Clone)]
pub struct AppState {
    pub db_pool: OraclePool,
    pub settings: Settings,
}


/// Handler for combobox metadata search
/// 
/// Accepts query parameter: ?search=<term>
/// Executes database query in a separate thread to avoid blocking Axum
pub async fn get_combobox_metadata(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    // Extract search term or use empty string if not provided
    let _search_term = params.search.unwrap_or_default();
    let pool = state.db_pool.clone();
    let _porta_do_servidor = &state.settings.server_address(); 

    // Execute heavy database query in a separate blocking thread
    let result = task::spawn_blocking(move || {
        let _conn = pool.get().map_err(|e| format!("Connection pool error: {}", e))?;
        
        // TODO: Implement your database query logic here
        // For now, returning a placeholder response
        Ok::<MetadataResponse, String>(MetadataResponse {
            results: vec![],
            total: 0,
        })
    })
    .await;

    // Handle tokio and database errors
    match result {
        Ok(Ok(data)) => (StatusCode::OK, Json(data)).into_response(),
        Ok(Err(db_err)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": db_err })),
        )
            .into_response(),
        Err(join_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": join_err.to_string() })),
        )
            .into_response(),
    }
}
