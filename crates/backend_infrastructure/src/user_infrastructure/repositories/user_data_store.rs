//! Data store SQL para usuarios.
//!
//! Encapsula queries concretas y devuelve `UserEntity` para que el adapter
//! de repositorio realice el mapeo al record de puerto.

use crate::user_infrastructure::repositories::user_entity::UserEntity;
use backend_application::user_ports::out::user::records::input::create_user_record::CreateUserRecord;
use backend_common::common::error::{CustomError, Result};
use crate::database::DbPool;
use uuid::Uuid;

pub struct UserDataStore {
    db_pool: DbPool,
}

impl UserDataStore {
    /// Inyecta el pool de DB que usara este data store.
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

impl UserDataStore {
    /// Inserta un usuario y devuelve la fila persistida.
    pub async fn create_user(&self, record: &CreateUserRecord) -> Result<UserEntity> {
        let query = r#"
            
INSERT INTO users (first_name, last_name, email, username, password)
VALUES ($1, $2, $3, $4, $5)
RETURNING user_id, first_name, last_name, email, username, password

"#;
        let result: UserEntity = sqlx::query_as::<_, UserEntity>(query)
            .bind(&record.first_name)
            .bind(&record.last_name)
            .bind(&record.email)
            .bind(&record.username)
            .bind(&record.password_hash)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|error| CustomError::DatabaseError(error.to_string()))?;
        Ok(result)
    }

    /// Lista usuarios desde DB.
    pub async fn list_users(&self) -> Result<Vec<UserEntity>> {
        let query = r#"
SELECT user_id, first_name, last_name, email, username, password
FROM users
"#;

        let result: Vec<UserEntity> = sqlx::query_as::<_, UserEntity>(query)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|error| CustomError::DatabaseError(error.to_string()))?;
        Ok(result)
    }

    /// Busca un usuario por `user_id`.
    pub async fn get_user(&self, user_id: &Uuid) -> Result<UserEntity> {
        let query = r#"
SELECT user_id, first_name, last_name, email, username, password
FROM users
WHERE user_id = $1
"#;
        let result: UserEntity = sqlx::query_as::<_, UserEntity>(query)
            .bind(&user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|error| CustomError::DatabaseError(error.to_string()))?;
        Ok(result)
    }
}
