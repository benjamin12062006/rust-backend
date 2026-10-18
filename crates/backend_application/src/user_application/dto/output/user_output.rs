use uuid::Uuid;

use backend_domain::user::user_model::User;

use crate::user_ports::out::user::records::output::stored_user_record::StoredUserRecord;

#[derive(Debug, Clone)]
pub struct UserOutput {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

impl From<StoredUserRecord> for UserOutput {
    fn from(value: StoredUserRecord) -> Self {
        Self {
            user_id: value.user_id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            username: value.username,
        }
    }
}

impl From<User> for UserOutput {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.user_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn stored_user_record() -> StoredUserRecord {
        StoredUserRecord {
            user_id: Uuid::from_u128(1),
            first_name: "Benji".to_string(),
            last_name: "Dev".to_string(),
            email: "benji@example.com".to_string(),
            username: "benji".to_string(),
            password_hash: "hashed-password".to_string(),
        }
    }

    #[fixture]
    fn domain_user() -> User {
        User {
            user_id: Uuid::from_u128(2),
            first_name: "Ana".to_string(),
            last_name: "Lopez".to_string(),
            email: "ana@example.com".to_string(),
            user_name: "ana".to_string(),
            password: "secret".to_string(),
        }
    }

    #[rstest]
    fn stored_user_record_maps_to_user_output_without_password(
        stored_user_record: StoredUserRecord,
    ) {
        let stored_user = stored_user_record;
        let output = UserOutput::from(stored_user);

        assert_eq!(output.user_id, Uuid::from_u128(1));
        assert_eq!(output.first_name, "Benji");
        assert_eq!(output.last_name, "Dev");
        assert_eq!(output.email, "benji@example.com");
        assert_eq!(output.username, "benji");
    }

    #[rstest]
    fn domain_user_maps_to_user_output(domain_user: User) {
        let output: UserOutput = domain_user.into();

        assert_eq!(output.user_id, Uuid::from_u128(2));
        assert_eq!(output.first_name, "Ana");
        assert_eq!(output.last_name, "Lopez");
        assert_eq!(output.email, "ana@example.com");
        assert_eq!(output.username, "ana");
    }
}
