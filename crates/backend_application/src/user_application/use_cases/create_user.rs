//! Caso de uso: crear usuario.
//!
//! Flujo: `CreateUserInput -> User -> CreateUserRecord -> repo -> UserOutput`.
//! Aqui se aplica hash de password antes de persistir.

use crate::user_application::dto::input::create_user_input::CreateUserInput;
use crate::user_application::dto::output::user_output::UserOutput;
use crate::user_application::policies::password_policy::hash_password;
use backend_domain::user::user_model::User;
use crate::user_ports::out::user::records::input::create_user_record::CreateUserRecord;
use crate::user_ports::out::user_repository::IUserRepository;
use backend_common::common::error::Result;

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
