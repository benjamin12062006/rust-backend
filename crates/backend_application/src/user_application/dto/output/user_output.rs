//! DTO de salida para exponer datos de usuario fuera de application.
//!
//! Flujo esperado en esta direccion:
//! - `User` (domain) -> `UserOutput` (application/output).
//! - No al reves, salvo necesidad explicita.
//!
//! Esto evita exponer directamente el modelo de dominio en adaptadores de salida
//! como HTTP.

use uuid::Uuid;

use crate::{
    features::user::domain::user_model::User,
    features::user::ports::out::user::records::output::stored_user_record::StoredUserRecord,
};

/// Datos de salida de usuario listos para capa externa.
///
/// Este DTO no expone campos sensibles como `password`.
#[derive(Debug, Clone)]
pub struct UserOutput {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

impl From<StoredUserRecord> for UserOutput {
    fn from(value: StoredUserRecord) -> Self {
        Self {
            user_id: value.user_id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            username: value.username,
        }
    }
}

/// Convierte el modelo de dominio `User` al DTO `UserOutput`.
///
/// Uso equivalente:
/// - `let output = UserOutput::from(user);`
/// - `let output: UserOutput = user.into();`
///
/// Regla mental:
/// - `impl From<A> for B` significa "A se convierte en B".
/// - Si existe `From<User> for UserOutput`, entonces `into()` se invoca sobre
///   `User` para obtener `UserOutput`.
///
/// Ejemplo:
/// ```rust
/// # use uuid::Uuid;
/// # struct User { user_id: Uuid, first_name: String, last_name: String, email: String, user_name: String, password: String }
/// # struct UserOutput { user_id: Uuid, first_name: String, last_name: String, email: String, username: String }
/// # impl From<User> for UserOutput {
/// #   fn from(user: User) -> Self {
/// #     Self { user_id: user.user_id, first_name: user.first_name, last_name: user.last_name, email: user.email, username: user.user_name }
/// #   }
/// # }
/// let user = User {
///     user_id: Uuid::nil(),
///     first_name: "Ana".to_string(),
///     last_name: "Lopez".to_string(),
///     email: "ana@example.com".to_string(),
///     user_name: "ana".to_string(),
///     password: "secret".to_string(),
/// };
///
/// let output = UserOutput::from(user);
/// # let _ = output;
/// ```
impl From<User> for UserOutput {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.user_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn stored_user_record() -> StoredUserRecord {
        StoredUserRecord {
            user_id: Uuid::from_u128(1),
            first_name: "Benji".to_string(),
            last_name: "Dev".to_string(),
            email: "benji@example.com".to_string(),
            username: "benji".to_string(),
            password_hash: "hashed-password".to_string(),
        }
    }

    #[fixture]
    fn domain_user() -> User {
        User {
            user_id: Uuid::from_u128(2),
            first_name: "Ana".to_string(),
            last_name: "Lopez".to_string(),
            email: "ana@example.com".to_string(),
            user_name: "ana".to_string(),
            password: "secret".to_string(),
        }
    }

    #[rstest]
    fn stored_user_record_maps_to_user_output_without_password(
        stored_user_record: StoredUserRecord,
    ) {
        let stored_user = stored_user_record;
        let output = UserOutput::from(stored_user);

        assert_eq!(output.user_id, Uuid::from_u128(1));
        assert_eq!(output.first_name, "Benji");
        assert_eq!(output.last_name, "Dev");
        assert_eq!(output.email, "benji@example.com");
        assert_eq!(output.username, "benji");
    }

    #[rstest]
    fn domain_user_maps_to_user_output(domain_user: User) {
        let output: UserOutput = domain_user.into();

        assert_eq!(output.user_id, Uuid::from_u128(2));
        assert_eq!(output.first_name, "Ana");
        assert_eq!(output.last_name, "Lopez");
        assert_eq!(output.email, "ana@example.com");
        assert_eq!(output.username, "ana");
    }
}
