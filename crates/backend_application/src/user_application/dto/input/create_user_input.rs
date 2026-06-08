//! DTO de entrada para el caso de uso de creacion de usuario.
//!
//! Este tipo vive en `application` porque representa datos que llegan al
//! caso de uso desde capas externas (por ejemplo HTTP).
//! No es el modelo de dominio: su responsabilidad es transportar datos
//! de entrada de forma explicita.
//!
//! Tambien define la conversion a `User` mediante `From<CreateUserInput>`.
//! Esa conversion funciona como puente entre capas y centraliza el mapeo
//! para no repetirlo en handlers, services o tests.

use uuid::Uuid;

use crate::features::user::domain::user_model::User;

/// Datos de entrada requeridos para crear un usuario.
///
/// Esta estructura modela el contrato de entrada del caso de uso.
/// En esta fase del backend mantenemos los campos simples y directos
/// para aprender la separacion entre DTO de application y entidad de dominio.
#[derive(Debug, Clone)]
pub struct CreateUserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    /// Contrasena en crudo al entrar al caso de uso.
    ///
    /// Regla esperada: hashear en application antes de persistir.
    pub password: String,
}

/// Convierte `CreateUserInput` en el modelo de dominio `User`.
///
/// Sentido arquitectonico:
/// - `CreateUserInput` pertenece a `application` (entrada).
/// - `User` pertenece a `domain` (modelo de negocio).
/// - Esta conversion explicita el paso entre capas.
///
/// Uso en codigo:
/// - `let user: User = User::from(input);`
/// - `let user: User = input.into();`
///
/// Ambas formas son equivalentes. `into()` funciona porque Rust implementa
/// `Into<User>` automaticamente cuando existe `From<CreateUserInput> for User`.
///
/// Nota sobre `user_id`:
/// se usa `Uuid::nil()` como placeholder en esta etapa.
/// Cuando el flujo definitivo este completo, el id real puede generarse en
/// application o en infrastructure/repository segun la decision de diseno.
impl From<CreateUserInput> for User {
    fn from(input: CreateUserInput) -> Self {
        Self {
            user_id: Uuid::nil(),
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
            user_name: input.username,
            password: input.password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_input_maps_to_domain_user() {
        let input = CreateUserInput {
            first_name: "Benji".to_string(),
            last_name: "Dev".to_string(),
            email: "benji@example.com".to_string(),
            username: "benji".to_string(),
            password: "secret".to_string(),
        };

        let user = User::from(input);

        assert_eq!(user.user_id, Uuid::nil());
        assert_eq!(user.first_name, "Benji");
        assert_eq!(user.last_name, "Dev");
        assert_eq!(user.email, "benji@example.com");
        assert_eq!(user.user_name, "benji");
        assert_eq!(user.password, "secret");
    }
}
