use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug, Clone, ThisError)]
pub enum CustomError {
    #[error("An error occurred during database interaction. {0}")]
    DatabaseError(String),

    #[error("An error occurred during http interaction. {0}")]
    HttpError(String),

    #[error("An error occurred while hashing password. {0}")]
    HashError(String),
}

impl warp::reject::Reject for CustomError {}

//hash errors
impl From<pwhash::error::Error> for CustomError {
    fn from(error: pwhash::error::Error) -> Self {
        CustomError::HashError(error.to_string())
    }
}
