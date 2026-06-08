use std::convert::Infallible;

use serde::Serialize;
use warp::http::StatusCode;
use warp::reject::{self};
use warp::reply::with_status;
use warp::{Reply, filters::body::BodyDeserializeError, reject::Rejection, reply::json};

use crate::shared::common::error::CustomError;
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(custom_error) = err.find::<CustomError>() {
        let status = match custom_error {
            CustomError::HttpError(_) => StatusCode::BAD_REQUEST,
            CustomError::DatabaseError(_) | CustomError::HashError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = json(&ErrorResponse {
            message: custom_error.to_string(),
        });
        return Ok(with_status(body, status));
    }

    if err.is_not_found() {
        let body = json(&ErrorResponse {
            message: "Route not found".to_string(),
        });
        return Ok(with_status(body, StatusCode::NOT_FOUND));
    }

    if err.find::<BodyDeserializeError>().is_some() {
        let body = json(&ErrorResponse {
            message: "Invalid request body".to_string(),
        });
        return Ok(with_status(body, StatusCode::BAD_REQUEST));
    }

    if err.find::<reject::UnsupportedMediaType>().is_some() {
        let body = json(&ErrorResponse {
            message: "Unsupported media type".to_string(),
        });
        return Ok(with_status(body, StatusCode::UNSUPPORTED_MEDIA_TYPE));
    }

    if err.find::<reject::MethodNotAllowed>().is_some() {
        let body = json(&ErrorResponse {
            message: "Method not allowed".to_string(),
        });
        return Ok(with_status(body, StatusCode::METHOD_NOT_ALLOWED));
    }

    let body = json(&ErrorResponse {
        message: "Unhandled rejection".to_string(),
    });
    Ok(with_status(body, StatusCode::INTERNAL_SERVER_ERROR))
}
