//! Entidad tecnica de infraestructura para SQLx.
//!
//! Representa la fila `users` tal como sale de la base y se mapea luego a
//! `StoredUserRecord` (frontera del puerto `out`).

use sqlx::FromRow;
use uuid::Uuid;

use crate::features::user::ports::out::user::records::output::stored_user_record::StoredUserRecord;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    user_id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    username: String,
    password: String,
}

/// Mapeo de entidad SQL -> record de salida del puerto de repositorio.
impl From<UserEntity> for StoredUserRecord {
    fn from(entity: UserEntity) -> Self {
        Self {
            user_id: entity.user_id,
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            username: entity.username,
            password_hash: entity.password,
        }
    }
}
