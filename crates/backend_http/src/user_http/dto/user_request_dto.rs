use serde::Deserialize;
use std::convert::TryFrom;

use crate::features::user::application::dto::input::create_user_input::CreateUserInput;
use crate::shared::common::error::CustomError;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl TryFrom<CreateUserDto> for CreateUserInput {
    type Error = CustomError;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        if dto.first_name.trim().is_empty() {
            return Err(CustomError::HttpError(
                "first_name must not be empty".to_string(),
            ));
        }
        if dto.last_name.trim().is_empty() {
            return Err(CustomError::HttpError(
                "last_name must not be empty".to_string(),
            ));
        }
        if dto.username.trim().is_empty() {
            return Err(CustomError::HttpError(
                "username must not be empty".to_string(),
            ));
        }
        if dto.password.trim().is_empty() {
            return Err(CustomError::HttpError(
                "password must not be empty".to_string(),
            ));
        }
        if dto.email.trim().is_empty() || !dto.email.contains('@') {
            return Err(CustomError::HttpError("email is not valid".to_string()));
        }

        Ok(CreateUserInput {
            first_name: dto.first_name,
            last_name: dto.last_name,
            email: dto.email,
            username: dto.username,
            password: dto.password,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn valid_dto() -> CreateUserDto {
        CreateUserDto {
            first_name: "Benji".to_string(),
            last_name: "Dev".to_string(),
            email: "benji@example.com".to_string(),
            username: "benji".to_string(),
            password: "secret".to_string(),
        }
    }

    #[rstest]
    fn create_user_dto_valid_becomes_create_user_input(valid_dto: CreateUserDto) {
        let input = CreateUserInput::try_from(valid_dto).expect("dto should be valid");

        assert_eq!(input.first_name, "Benji");
        assert_eq!(input.last_name, "Dev");
        assert_eq!(input.email, "benji@example.com");
        assert_eq!(input.username, "benji");
        assert_eq!(input.password, "secret");
    }

    #[rstest]
    #[case::empty_first_name("first_name", " ", "first_name must not be empty")]
    #[case::empty_last_name("last_name", " ", "last_name must not be empty")]
    #[case::empty_username("username", " ", "username must not be empty")]
    #[case::empty_password("password", " ", "password must not be empty")]
    #[case::invalid_email("email", "not-an-email", "email is not valid")]
    fn create_user_dto_rejects_invalid_fields(
        #[case] field: &str,
        #[case] value: &str,
        #[case] expected_message: &str,
        valid_dto: CreateUserDto,
    ) {
        let mut dto = valid_dto;
        match field {
            "first_name" => dto.first_name = value.to_string(),
            "last_name" => dto.last_name = value.to_string(),
            "username" => dto.username = value.to_string(),
            "password" => dto.password = value.to_string(),
            "email" => dto.email = value.to_string(),
            _ => unreachable!("unknown dto field in test case"),
        }

        let result = CreateUserInput::try_from(dto);

        assert!(matches!(
            result,
            Err(CustomError::HttpError(message)) if message.contains(expected_message)
        ));
    }
}
