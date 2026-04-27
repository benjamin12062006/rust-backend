//! Conexion tecnica a Postgres mediante SQLx.

pub mod connection;
pub mod health;

pub use connection::{DbPool, create_connection};
pub use health::check_db_connection;
