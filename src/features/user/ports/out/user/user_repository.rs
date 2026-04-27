//! Puerto de salida para persistencia/lectura de usuarios.

use async_trait::async_trait;
use uuid::Uuid;

use crate::features::user::ports::out::user::records::input::create_user_record::CreateUserRecord;
use crate::features::user::ports::out::user::records::output::stored_user_record::StoredUserRecord;
use crate::shared::common::error::Result;

/// Contrato que application usa para hablar con infraestructura de usuarios.
#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn create_user(&self, record: &CreateUserRecord) -> Result<StoredUserRecord>;
    async fn list_users(&self) -> Result<Vec<StoredUserRecord>>;
    async fn get_user(&self, id: &Uuid) -> Result<StoredUserRecord>;
}
