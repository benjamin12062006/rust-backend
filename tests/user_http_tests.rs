mod support;

use backend::shared::bootstrap::http_server::{self, Services};
use backend::shared::bootstrap::middleware;
use serde_json::{Value, json};
use support::builders;
use support::fakes::FakeUserUseCase;
use support::runtime::block_on;
use uuid::Uuid;
use warp::Filter;
use warp::http::StatusCode;

#[test]
fn get_users_returns_json_users_without_database() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.into_arc(),
        })
        .recover(middleware::error::handle_rejection);

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

#[test]
fn post_users_creates_user_through_inbound_port() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.clone().into_arc(),
        })
        .recover(middleware::error::handle_rejection);

        let response = warp::test::request()
            .method("POST")
            .path("/v1/users")
            .header("content-type", "application/json")
            .json(&json!({
                "first_name": "Benji",
                "last_name": "Dev",
                "email": "benji@example.com",
                "username": "benji",
                "password": "plain-password"
            }))
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

#[test]
fn post_users_rejects_invalid_email_before_calling_use_case() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.clone().into_arc(),
        })
        .recover(middleware::error::handle_rejection);

        let response = warp::test::request()
            .method("POST")
            .path("/v1/users")
            .header("content-type", "application/json")
            .json(&json!({
                "first_name": "Benji",
                "last_name": "Dev",
                "email": "invalid-email",
                "username": "benji",
                "password": "plain-password"
            }))
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(fake.created_inputs().is_empty());

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert!(
            body["message"]
                .as_str()
                .unwrap()
                .contains("email is not valid")
        );
    });
}

#[test]
fn post_users_rejects_invalid_json_body() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.clone().into_arc(),
        })
        .recover(middleware::error::handle_rejection);

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

#[test]
fn get_user_returns_one_user_and_passes_uuid_to_use_case() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.clone().into_arc(),
        })
        .recover(middleware::error::handle_rejection);

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

#[test]
fn unsupported_user_method_returns_controlled_json_405() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.into_arc(),
        })
        .recover(middleware::error::handle_rejection);

        let response = warp::test::request()
            .method("DELETE")
            .path("/v1/users")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["message"], "Method not allowed");
    });
}

#[test]
fn unknown_route_returns_controlled_json_404() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.into_arc(),
        })
        .recover(middleware::error::handle_rejection);

        let response = warp::test::request()
            .method("GET")
            .path("/unknown")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["message"], "Route not found");
    });
}

#[test]
fn invalid_uuid_path_returns_controlled_json_404() {
    block_on(async {
        let fake = FakeUserUseCase::new(builders::user_output());
        let routes = http_server::routes(Services {
            user_service: fake.into_arc(),
        })
        .recover(middleware::error::handle_rejection);

        let response = warp::test::request()
            .method("GET")
            .path("/v1/users/not-a-uuid")
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body: Value = serde_json::from_slice(response.body()).expect("response should be json");
        assert_eq!(body["message"], "Route not found");
    });
}

#[test]
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
        let routes = http_server::routes(Services {
            user_service: fake.clone().into_arc(),
        })
        .recover(middleware::error::handle_rejection);

        let response = warp::test::request()
            .method("GET")
            .path(&format!("/v1/users/{id}"))
            .reply(&routes)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(fake.last_get_id(), Some(id));
    });
}
