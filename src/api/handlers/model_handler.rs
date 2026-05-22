use axum::{extract::State, Json};
use std::sync::Arc;
use crate::service::model_service::ModelService;
use crate::error::AppError;
use crate::domain::models::model::LlmInfo;
use crate::api::handlers::metadata_handler::AppState;
use serde::Serialize;

#[derive(Serialize)]
pub struct ModelResponse {
    pub models: Vec<LlmInfo>,
}

pub async fn list_models_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ModelResponse>, AppError> {
    let models = ModelService::get_llm_info(&state.db_pool)?;

    Ok(Json(ModelResponse { models }))
}