//! Fachada de application para usuario.
//!
//! `UserService` implementa el puerto de entrada (`IUserUseCase`) y delega la
//! logica real en `use_cases`. Asi mantenemos casos de uso aislados y
//! reutilizables.

use async_trait::async_trait;
use uuid::Uuid;

use crate::user_application::dto::input::create_user_input::CreateUserInput;
use crate::user_application::dto::output::user_output::UserOutput;
use crate::user_application::use_cases;
use crate::user_ports::inbound::user_use_case::IUserUseCase;
use crate::user_ports::out::user_repository::IUserRepository;
use backend_common::common::error::Result;

/// Adaptador de application que conecta puerto de entrada con casos de uso.
pub struct UserService<R>
where
    R: IUserRepository,
{
    user_repostory: R,
}

impl<R> UserService<R>
where
    R: IUserRepository,
{
    /// Crea la fachada con la implementacion concreta del repositorio.
    pub fn new(user_repostory: R) -> Self {
        Self { user_repostory }
    }
}

/// Implementacion del puerto de entrada con delegacion a `use_cases`.
#[async_trait]
impl<R> IUserUseCase for UserService<R>
where
    R: IUserRepository + Send + Sync,
{
    async fn create_user(&self, input: CreateUserInput) -> Result<UserOutput> {
        use_cases::create_user::execute(&self.user_repostory, input).await
    }

    async fn list_users(&self) -> Result<Vec<UserOutput>> {
        use_cases::list_users::execute(&self.user_repostory).await
    }

    async fn get_user(&self, id: &Uuid) -> Result<UserOutput> {
        use_cases::get_user::execute(&self.user_repostory, id).await
    }
}
