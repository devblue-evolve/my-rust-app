use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Database connection error: {0}")]
    DatabaseError(#[from] oracle::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Model not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal server error")]
    Internal(String),

    #[error("Unknown error occurred")]
    Unknown,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        let (status, error_type, details) = match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND".to_string(),
                Some(msg),
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR".to_string(),
                Some(msg),
            ),
            AppError::ConfigError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CONFIG_ERROR".to_string(),
                Some(msg),
            ),
            AppError::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR".to_string(),
                Some(e.to_string()),
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR".to_string(),
                Some(msg),
            ),
            AppError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "UNKNOWN_ERROR".to_string(),
                None,
            ),
        };

        let error_response = ErrorResponse {
            error: error_type,
            message,
            details,
        };

        (status, Json(error_response)).into_response()
    }
}
