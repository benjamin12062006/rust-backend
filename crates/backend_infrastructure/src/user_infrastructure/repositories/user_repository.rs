//! Adapter de repositorio de usuario.
//!
//! Implementa el puerto `IUserRepository` usando `UserDataStore` (SQLx) y
//! mapea `UserEntity` a `StoredUserRecord`.

use crate::features::user::infrastructure::repositories::user_data_store::UserDataStore;
use crate::features::user::infrastructure::repositories::user_entity::UserEntity;
use crate::features::user::ports::out::user::records::input::create_user_record::CreateUserRecord;
use crate::features::user::ports::out::user::records::output::stored_user_record::StoredUserRecord;
use crate::features::user::ports::out::user::user_repository::IUserRepository;
use crate::shared::common::error::CustomError;
use async_trait::async_trait;

pub struct UserRepository {
    data_store: UserDataStore,
}

impl UserRepository {
    /// Crea el adapter con su data store concreto.
    pub fn new(data_store: UserDataStore) -> Self {
        Self { data_store }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create_user(
        &self,
        record: &CreateUserRecord,
    ) -> Result<StoredUserRecord, CustomError> {
        let result: UserEntity = self.data_store.create_user(record).await?;
        Ok(StoredUserRecord::from(result))
    }

    async fn list_users(&self) -> Result<Vec<StoredUserRecord>, CustomError> {
        let result: Vec<UserEntity> = self.data_store.list_users().await?;
        Ok(result.into_iter().map(StoredUserRecord::from).collect())
    }

    async fn get_user(&self, user_id: &uuid::Uuid) -> Result<StoredUserRecord, CustomError> {
        let result: UserEntity = self.data_store.get_user(user_id).await?;
        Ok(StoredUserRecord::from(result))
    }
}
