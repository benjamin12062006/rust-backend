//! Caso de uso: crear usuario.
//!
//! Flujo: `CreateUserInput -> User -> CreateUserRecord -> repo -> UserOutput`.
//! Aqui se aplica hash de password antes de persistir.

use crate::features::user::application::dto::input::create_user_input::CreateUserInput;
use crate::features::user::application::dto::output::user_output::UserOutput;
use crate::features::user::application::policies::password_policy::hash_password;
use crate::features::user::domain::user_model::User;
use crate::features::user::ports::out::user::records::input::create_user_record::CreateUserRecord;
use crate::features::user::ports::out::user_repository::IUserRepository;
use crate::shared::common::error::Result;

/// Ejecuta el caso de uso de creacion de usuario.
pub async fn execute<R>(repo: &R, input: CreateUserInput) -> Result<UserOutput>
where
    R: IUserRepository + Send + Sync,
{
    let mut new_user = User::from(input);
    let password_hash = hash_password(&new_user.password)?;
    new_user.password = password_hash;

    let recort = CreateUserRecord::from(new_user);
    let stored_user = repo.create_user(&recort).await?;
    Ok(UserOutput::from(stored_user))
}
