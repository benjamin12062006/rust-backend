mod support;

use std::convert::Infallible;

use backend::shared::bootstrap::http_server::{self, Services};
use backend::shared::bootstrap::middleware;
use rstest::{fixture, rstest};
use serde_json::{Value, json};
use support::builders;
use support::fakes::FakeUserUseCase;
use support::runtime::block_on;
use uuid::Uuid;
use warp::Filter;
use warp::http::StatusCode;

#[fixture]
fn fake() -> FakeUserUseCase {
    FakeUserUseCase::new(builders::user_output())
}

fn routes_for(
    fake: FakeUserUseCase,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    http_server::routes(Services {
        user_service: fake.into_arc(),
    })
    .recover(middleware::error::handle_rejection)
}

fn valid_create_user_payload() -> Value {
    json!({
        "first_name": "Benji",
        "last_name": "Dev",
        "email": "benji@example.com",
        "username": "benji",
        "password": "plain-password"
    })
}

#[rstest]
fn get_users_returns_json_users_without_database(fake: FakeUserUseCase) {
    block_on(async {
        let routes = routes_for(fake);

        let response = warp::test::request()
            .method("GET")
            .path("/v1/users")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body[0]["user_id"], builders::DEFAULT_USER_ID.to_string());
        assert_eq!(body[0]["username"], "benji");
    });
}

#[rstest]
fn post_users_creates_user_through_inbound_port(fake: FakeUserUseCase) {
    block_on(async {
        let routes = routes_for(fake.clone());

        let response = warp::test::request()
            .method("POST")
            .path("/v1/users")
            .header("content-type", "application/json")
            .json(&valid_create_user_payload())
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["user_id"], builders::DEFAULT_USER_ID.to_string());
        assert_eq!(body["username"], "benji");

        let created_inputs = fake.created_inputs();
        assert_eq!(created_inputs.len(), 1);
        assert_eq!(created_inputs[0].email, "benji@example.com");
        assert_eq!(created_inputs[0].password, "plain-password");
    });
}

#[rstest]
#[case::empty_first_name("first_name", " ", "first_name must not be empty")]
#[case::empty_last_name("last_name", " ", "last_name must not be empty")]
#[case::empty_username("username", " ", "username must not be empty")]
#[case::empty_password("password", " ", "password must not be empty")]
#[case::invalid_email("email", "invalid-email", "email is not valid")]
fn post_users_rejects_invalid_payload_before_calling_use_case(
    #[case] field: &str,
    #[case] value: &str,
    #[case] expected_message: &str,
    fake: FakeUserUseCase,
) {
    block_on(async {
        let routes = routes_for(fake.clone());
        let mut payload = valid_create_user_payload();
        payload[field] = json!(value);

        let response = warp::test::request()
            .method("POST")
            .path("/v1/users")
            .header("content-type", "application/json")
            .json(&payload)
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(fake.created_inputs().is_empty());

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert!(body["message"].as_str().unwrap().contains(expected_message));
    });
}

#[rstest]
fn post_users_rejects_invalid_json_body(fake: FakeUserUseCase) {
    block_on(async {
        let routes = routes_for(fake.clone());

        let response = warp::test::request()
            .method("POST")
            .path("/v1/users")
            .header("content-type", "application/json")
            .body("{ invalid-json")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(fake.created_inputs().is_empty());

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["message"], "Invalid request body");
    });
}

#[rstest]
fn get_user_returns_one_user_and_passes_uuid_to_use_case(fake: FakeUserUseCase) {
    block_on(async {
        let routes = routes_for(fake.clone());

        let response = warp::test::request()
            .method("GET")
            .path(&format!("/v1/users/{}", builders::DEFAULT_USER_ID))
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(fake.last_get_id(), Some(builders::DEFAULT_USER_ID));

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["user_id"], builders::DEFAULT_USER_ID.to_string());
        assert_eq!(body["username"], "benji");
    });
}

#[rstest]
#[case::unsupported_user_method(
    "DELETE",
    "/v1/users",
    StatusCode::METHOD_NOT_ALLOWED,
    "Method not allowed"
)]
#[case::unknown_route("GET", "/unknown", StatusCode::NOT_FOUND, "Route not found")]
#[case::invalid_uuid_path(
    "GET",
    "/v1/users/not-a-uuid",
    StatusCode::NOT_FOUND,
    "Route not found"
)]
fn route_rejections_return_controlled_json(
    #[case] method: &str,
    #[case] path: &str,
    #[case] expected_status: StatusCode,
    #[case] expected_message: &str,
    fake: FakeUserUseCase,
) {
    block_on(async {
        let routes = routes_for(fake);

        let response = warp::test::request()
            .method(method)
            .path(path)
            .reply(&routes)
            .await;

        assert_eq!(response.status(), expected_status);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["message"], expected_message);
    });
}

#[rstest]
fn routes_accept_any_valid_uuid_shape() {
    block_on(async {
        let id = Uuid::from_u128(99);
        let output = backend::features::user::application::dto::output::user_output::UserOutput {
            user_id: id,
            first_name: "Route".to_string(),
            last_name: "Test".to_string(),
            email: "route@example.com".to_string(),
            username: "route-test".to_string(),
        };
        let fake = FakeUserUseCase::new(output);
        let routes = routes_for(fake.clone());

        let response = warp::test::request()
            .method("GET")
            .path(&format!("/v1/users/{id}"))
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(fake.last_get_id(), Some(id));
    });
}
