use crate::error::AppError;
use std::env;

#[derive(Clone, Debug)]
pub struct Settings {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub db_user: String,
    pub db_password: String,
    pub log_level: String,
    pub environment: String,
}

impl Settings {
    pub fn from_env() -> Result<Self, AppError> {
        let server_host =
            env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port: u16 = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| AppError::ConfigError("Invalid SERVER_PORT".to_string()))?;

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::ConfigError("DATABASE_URL not set".to_string()))?;
        let db_user =
            env::var("DB_USER").map_err(|_| AppError::ConfigError("DB_USER not set".to_string()))?;
        let db_password = env::var("DB_PASSWORD")
            .map_err(|_| AppError::ConfigError("DB_PASSWORD not set".to_string()))?;

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

        Ok(Settings {
            server_host,
            server_port,
            database_url,
            db_user,
            db_password,
            log_level,
            environment,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
