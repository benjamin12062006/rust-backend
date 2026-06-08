//! Caso de uso: obtener un usuario por id.
//!
//! Lee desde repositorio y adapta el resultado al DTO de salida.

use uuid::Uuid;

use crate::features::user::application::dto::output::user_output::UserOutput;
use crate::features::user::ports::out::user::user_repository::IUserRepository;
use crate::shared::common::error::Result;

/// Ejecuta el caso de uso de busqueda por id.
pub async fn execute<R>(repo: &R, id: &Uuid) -> Result<UserOutput>
where
    R: IUserRepository + Send + Sync,
{
    let stores_user = repo.get_user(id).await?;
    Ok(UserOutput::from(stores_user))
}
