use crate::repository::model_repo::ModelRepository;
use crate::domain::models::model::LlmInfo;
use crate::error::AppError;
use crate::db::connection::OraclePool;

pub struct ModelService;

impl ModelService {
    pub fn get_llm_info(pool: &OraclePool) -> Result<Vec<LlmInfo>, AppError> {
        let conn = pool.get().map_err(|e| {
            AppError::ConfigError(format!("Failed to get connection from pool: {}", e))
        })?;
        ModelRepository::fetch_all_models(&conn).map_err(AppError::from)
    }
}