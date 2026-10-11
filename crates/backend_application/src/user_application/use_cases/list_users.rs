//! Caso de uso: listar usuarios.
//!
//! Convierte la salida del repositorio (`StoredUserRecord`) al DTO de salida
//! (`UserOutput`) para cumplir el contrato del puerto de entrada.

use crate::user_application::dto::output::user_output::UserOutput;
use crate::user_ports::out::user::user_repository::IUserRepository;
use backend_common::common::error::Result;

/// Ejecuta el caso de uso de listado de usuarios.
pub async fn execute<R>(repo: &R) -> Result<Vec<UserOutput>>
where
    R: IUserRepository + Send + Sync,
{
    let stored_users = repo.list_users().await?;
    Ok(stored_users.into_iter().map(UserOutput::from).collect())
}
