//! Record de entrada al repositorio para crear usuario.

use crate::features::user::domain::user_model::User;

/// Datos que cruzan desde application hacia el repositorio al crear usuario.
#[derive(Debug, Clone)]
pub struct CreateUserRecord {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

/// Mapeo desde dominio hacia record de frontera del puerto `out`.
impl From<User> for CreateUserRecord {
    fn from(user: User) -> Self {
        Self {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.user_name,
            password_hash: user.password,
        }
    }
}
