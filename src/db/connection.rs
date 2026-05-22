use crate::error::AppError;
use crate::config::Settings;
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;

pub type OraclePool = Pool<OracleConnectionManager>;

/// Creates and configures a database connection pool
/// 
/// Default configuration:
/// - 5 minimum idle connections
/// - 20 maximum connections
pub fn establish_pool(settings: &Settings) -> Result<OraclePool, AppError> {
    let manager = OracleConnectionManager::new(
        &settings.db_user,
        &settings.db_password,
        &settings.database_url,
    );

    // Create pool with default configuration
    Pool::new(manager).map_err(|e| AppError::ConfigError(format!("Failed to create connection pool: {}", e)))
}