use serde::Serialize;
use uuid::Uuid;

use crate::features::user::application::dto::output::user_output::UserOutput;

#[derive(Serialize)]
pub struct UserResponseDto {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

impl From<UserOutput> for UserResponseDto {
    fn from(output: UserOutput) -> Self {
        Self {
            user_id: output.user_id,
            first_name: output.first_name,
            last_name: output.last_name,
            email: output.email,
            username: output.username,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn user_output() -> UserOutput {
        UserOutput {
            user_id: Uuid::from_u128(1),
            first_name: "Benji".to_string(),
            last_name: "Dev".to_string(),
            email: "benji@example.com".to_string(),
            username: "benji".to_string(),
        }
    }

    #[rstest]
    fn user_output_maps_to_http_response_dto(user_output: UserOutput) {
        let output = user_output;
        let response = UserResponseDto::from(output);

        assert_eq!(response.user_id, Uuid::from_u128(1));
        assert_eq!(response.first_name, "Benji");
        assert_eq!(response.last_name, "Dev");
        assert_eq!(response.email, "benji@example.com");
        assert_eq!(response.username, "benji");
    }
}
