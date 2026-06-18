//! Contrato de entrada para casos de uso de usuario.

use async_trait::async_trait;
use uuid::Uuid;

use crate::user_application::dto::input::create_user_input::CreateUserInput;
use crate::user_application::dto::output::user_output::UserOutput;
use backend_common::common::error::Result;

/// Puerto de entrada que expone operaciones de usuario para capas externas.
#[async_trait]
pub trait IUserUseCase: Send + Sync {
    async fn create_user(&self, input: CreateUserInput) -> Result<UserOutput>;
    async fn list_users(&self) -> Result<Vec<UserOutput>>;
    async fn get_user(&self, id: &Uuid) -> Result<UserOutput>;
}
