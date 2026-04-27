use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(&'static str),
}
