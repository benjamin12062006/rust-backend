mod support;

use std::convert::Infallible;

use backend::shared::bootstrap::middleware;
use backend::shared::common::error::CustomError;
use rstest::rstest;
use serde_json::Value;
use support::runtime::block_on;
use warp::http::StatusCode;
use warp::{Filter, Rejection, reject};

fn custom_error_route(
    error: CustomError,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    warp::any()
        .and_then(move || {
            let error = error.clone();
            async move { Err::<warp::reply::Response, Rejection>(reject::custom(error)) }
        })
        .recover(middleware::error::handle_rejection)
}

#[rstest]
#[case::http_error(
    CustomError::HttpError("bad request".to_string()),
    StatusCode::BAD_REQUEST,
    "bad request"
)]
#[case::database_error(
    CustomError::DatabaseError("database failed".to_string()),
    StatusCode::INTERNAL_SERVER_ERROR,
    "database failed"
)]
#[case::hash_error(
    CustomError::HashError("hash failed".to_string()),
    StatusCode::INTERNAL_SERVER_ERROR,
    "hash failed"
)]
fn handle_rejection_maps_custom_errors_to_json_response(
    #[case] error: CustomError,
    #[case] expected_status: StatusCode,
    #[case] expected_message: &str,
) {
    block_on(async {
        let route = custom_error_route(error);

        let response = warp::test::request().reply(&route).await;

        assert_eq!(response.status(), expected_status);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert!(body["message"].as_str().unwrap().contains(expected_message));
    });
}

#[rstest]
fn handle_rejection_maps_not_found_to_404() {
    block_on(async {
        let route = warp::path("known")
            .map(|| "ok")
            .recover(middleware::error::handle_rejection);

        let response = warp::test::request().path("/missing").reply(&route).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["message"], "Route not found");
    });
}

#[rstest]
fn with_service_injects_service_into_warp_filter() {
    block_on(async {
        let route = middleware::service::with_service("user-service".to_string())
            .map(|service: String| format!("service={service}"));

        let response = warp::test::request().path("/").reply(&route).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body().as_ref(), b"service=user-service");
    });
}
