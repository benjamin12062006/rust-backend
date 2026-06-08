mod support;

use backend::shared::bootstrap::middleware;
use backend::shared::common::error::CustomError;
use serde_json::Value;
use support::runtime::block_on;
use warp::http::StatusCode;
use warp::{Filter, Rejection, reject};

#[test]
fn handle_rejection_maps_http_error_to_400() {
    block_on(async {
        let route = warp::any()
            .and_then(|| async {
                Err::<warp::reply::Response, Rejection>(reject::custom(CustomError::HttpError(
                    "bad request".to_string(),
                )))
            })
            .recover(middleware::error::handle_rejection);

        let response = warp::test::request().reply(&route).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert!(body["message"].as_str().unwrap().contains("bad request"));
    });
}

#[test]
fn handle_rejection_maps_database_error_to_500() {
    block_on(async {
        let route = warp::any()
            .and_then(|| async {
                Err::<warp::reply::Response, Rejection>(reject::custom(CustomError::DatabaseError(
                    "database failed".to_string(),
                )))
            })
            .recover(middleware::error::handle_rejection);

        let response = warp::test::request().reply(&route).await;

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    });
}

#[test]
fn handle_rejection_maps_hash_error_to_500() {
    block_on(async {
        let route = warp::any()
            .and_then(|| async {
                Err::<warp::reply::Response, Rejection>(reject::custom(CustomError::HashError(
                    "hash failed".to_string(),
                )))
            })
            .recover(middleware::error::handle_rejection);

        let response = warp::test::request().reply(&route).await;

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    });
}

#[test]
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

#[test]
fn with_service_injects_service_into_warp_filter() {
    block_on(async {
        let route = middleware::service::with_service("user-service".to_string())
            .map(|service: String| format!("service={service}"));

        let response = warp::test::request().path("/").reply(&route).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body().as_ref(), b"service=user-service");
    });
}
