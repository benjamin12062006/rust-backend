//! Record de salida del repositorio tras crear/leer usuario.

use uuid::Uuid;

/// Datos persistidos que infraestructura devuelve a application.
#[derive(Debug, Clone)]
pub struct StoredUserRecord {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
