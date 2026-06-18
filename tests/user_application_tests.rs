mod support;

use backend_application::user_application::use_cases;
use backend_application::user_application::user_service::UserService;
use backend_application::user_ports::inbound::user_use_case::IUserUseCase;
use backend_common::common::error::CustomError;
use pwhash::bcrypt;
use rstest::{fixture, rstest};
use support::builders;
use support::fakes::FakeUserRepository;
use support::runtime::block_on;
use uuid::Uuid;

#[fixture]
fn repo() -> FakeUserRepository {
    FakeUserRepository::default()
}

#[fixture]
fn repo_with_stored_user() -> FakeUserRepository {
    FakeUserRepository::default().with_create_result(Ok(builders::stored_user_record()))
}

#[rstest]
fn create_user_hashes_password_before_persisting(repo: FakeUserRepository) {
    block_on(async {
        let input = builders::create_user_input();

        let output = use_cases::create_user::execute(&repo, input)
            .await
            .expect("user should be created");

        let created_records = repo.created_records();
        assert_eq!(created_records.len(), 1);

        let record = &created_records[0];
        assert_eq!(record.first_name, "Benji");
        assert_eq!(record.last_name, "Dev");
        assert_eq!(record.email, "benji@example.com");
        assert_eq!(record.username, "benji");
        assert_ne!(record.password_hash, "plain-password");
        assert!(bcrypt::verify("plain-password", &record.password_hash));

        assert_eq!(output.user_id, Uuid::from_u128(1));
        assert_eq!(output.username, "benji");
    });
}

#[rstest]
fn create_user_returns_user_output_from_repository_record(
    repo_with_stored_user: FakeUserRepository,
) {
    block_on(async {
        let output =
            use_cases::create_user::execute(&repo_with_stored_user, builders::create_user_input())
                .await
                .expect("user should be created");

        assert_eq!(output.user_id, builders::DEFAULT_USER_ID);
        assert_eq!(output.first_name, "Benji");
        assert_eq!(output.last_name, "Dev");
        assert_eq!(output.email, "benji@example.com");
        assert_eq!(output.username, "benji");
    });
}

#[rstest]
fn create_user_propagates_repository_error() {
    block_on(async {
        let repo = FakeUserRepository::default()
            .with_create_result(Err(CustomError::DatabaseError("insert failed".to_string())));

        let result = use_cases::create_user::execute(&repo, builders::create_user_input()).await;

        assert!(matches!(result, Err(CustomError::DatabaseError(_))));
    });
}

#[rstest]
fn list_users_returns_outputs_from_repository_records() {
    block_on(async {
        let repo = FakeUserRepository::default()
            .with_list_result(Ok(vec![builders::stored_user_record()]));

        let users = use_cases::list_users::execute(&repo)
            .await
            .expect("users should be listed");

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_id, builders::DEFAULT_USER_ID);
        assert_eq!(users[0].username, "benji");
    });
}

#[rstest]
fn list_users_returns_empty_vec_when_repository_is_empty(repo: FakeUserRepository) {
    block_on(async {
        let users = use_cases::list_users::execute(&repo)
            .await
            .expect("empty list should be valid");

        assert!(users.is_empty());
    });
}

#[rstest]
fn list_users_propagates_repository_error() {
    block_on(async {
        let repo = FakeUserRepository::default()
            .with_list_result(Err(CustomError::DatabaseError("select failed".to_string())));

        let result = use_cases::list_users::execute(&repo).await;

        assert!(matches!(result, Err(CustomError::DatabaseError(_))));
    });
}

#[rstest]
fn get_user_returns_matching_user_and_borrows_uuid_to_repository() {
    block_on(async {
        let repo =
            FakeUserRepository::default().with_get_result(Ok(builders::stored_user_record()));
        let id = builders::DEFAULT_USER_ID;

        let output = use_cases::get_user::execute(&repo, &id)
            .await
            .expect("user should be found");

        assert_eq!(repo.last_get_id(), Some(id));
        assert_eq!(output.user_id, id);
        assert_eq!(output.username, "benji");
    });
}

#[rstest]
fn get_user_propagates_repository_error() {
    block_on(async {
        let repo = FakeUserRepository::default()
            .with_get_result(Err(CustomError::HttpError("user not found".to_string())));

        let result = use_cases::get_user::execute(&repo, &Uuid::from_u128(404)).await;

        assert!(matches!(result, Err(CustomError::HttpError(_))));
    });
}

#[rstest]
fn user_service_implements_inbound_port_over_repository_port() {
    block_on(async {
        let repo = FakeUserRepository::default()
            .with_list_result(Ok(vec![builders::stored_user_record()]));
        let service = UserService::new(repo);

        let users = service
            .list_users()
            .await
            .expect("service should delegate to list use case");

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "benji");
    });
}
