use uuid::Uuid;

use backend_domain::user::user_model::User;

#[derive(Debug, Clone)]
pub struct CreateUserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl From<CreateUserInput> for User {
    fn from(input: CreateUserInput) -> Self {
        Self {
            user_id: Uuid::nil(),
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
            user_name: input.username,
            password: input.password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn create_user_input() -> CreateUserInput {
        CreateUserInput {
            first_name: "Benji".to_string(),
            last_name: "Dev".to_string(),
            email: "benji@example.com".to_string(),
            username: "benji".to_string(),
            password: "secret".to_string(),
        }
    }

    #[rstest]
    fn create_user_input_maps_to_domain_user(create_user_input: CreateUserInput) {
        let input = create_user_input;
        let user = User::from(input);

        assert_eq!(user.user_id, Uuid::nil());
        assert_eq!(user.first_name, "Benji");
        assert_eq!(user.last_name, "Dev");
        assert_eq!(user.email, "benji@example.com");
        assert_eq!(user.user_name, "benji");
        assert_eq!(user.password, "secret");
    }
}
