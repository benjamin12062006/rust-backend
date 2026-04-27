use sqlx::Error as SqlxError;
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

impl From<SqlxError> for CustomError {
    fn from(sqlx_error: SqlxError) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => CustomError::DatabaseError(db_error.to_string()),
            None => {
                eprintln!("error {:?}", sqlx_error);
                CustomError::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}



impl From<surrealdb::Error> for CustomError {
    fn from(error: surrealdb::Error) -> Self {
        CustomError::DatabaseError(error.to_string())
    }
}

impl From<qdrant_client::QdrantError> for CustomError  {
    fn from(error: qdrant_client::QdrantError) -> Self {
        CustomError::DatabaseError(error.to_string())
    }
}


impl warp::reject::Reject for CustomError {}

//hash errors
impl From<pwhash::error::Error> for CustomError {
    fn from(error: pwhash::error::Error) -> Self {
        CustomError::HashError(error.to_string())
    }
}
