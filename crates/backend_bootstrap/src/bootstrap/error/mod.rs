use thiserror::Error as ThisError;

use backend_common::common::error::CustomError;
use backend_config::configuration::error::config_error::ConfigError;

#[derive(Debug, ThisError)]
pub enum BootstrapError {
    #[error("Configuration error: {0}")]
    Configuration(#[from] ConfigError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] CustomError),

    #[error("Database migration error: {0}")]
    Migration(String),

    #[error("Runtime error: {0}")]
    Runtime(#[from] std::io::Error),

    #[error("Invalid server port: {0}")]
    InvalidPort(String),

    #[error("Missing bootstrap resource `{resource}` for feature `{feature}`")]
    MissingFeatureResource {
        feature: &'static str,
        resource: &'static str,
    },
}
