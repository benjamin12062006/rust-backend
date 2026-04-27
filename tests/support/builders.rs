use backend::features::user::application::dto::input::create_user_input::CreateUserInput;
use backend::features::user::application::dto::output::user_output::UserOutput;
use backend::features::user::ports::out::user::records::output::stored_user_record::StoredUserRecord;
use uuid::Uuid;

pub const DEFAULT_USER_ID: Uuid = Uuid::from_u128(1);

pub fn create_user_input() -> CreateUserInput {
    CreateUserInput {
        first_name: "Benji".to_string(),
        last_name: "Dev".to_string(),
        email: "benji@example.com".to_string(),
        username: "benji".to_string(),
        password: "plain-password".to_string(),
    }
}

pub fn stored_user_record() -> StoredUserRecord {
    StoredUserRecord {
        user_id: DEFAULT_USER_ID,
        first_name: "Benji".to_string(),
        last_name: "Dev".to_string(),
        email: "benji@example.com".to_string(),
        username: "benji".to_string(),
        password_hash: "$2y$hash-from-test".to_string(),
    }
}

pub fn user_output() -> UserOutput {
    UserOutput {
        user_id: DEFAULT_USER_ID,
        first_name: "Benji".to_string(),
        last_name: "Dev".to_string(),
        email: "benji@example.com".to_string(),
        username: "benji".to_string(),
    }
}
