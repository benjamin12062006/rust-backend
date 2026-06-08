//! Conexion tecnica a SurrealDB mediante su SDK oficial.

pub mod connection;
pub mod health;

pub use connection::{SurrealDbClient, create_connection};
pub use health::check_db_connection;
