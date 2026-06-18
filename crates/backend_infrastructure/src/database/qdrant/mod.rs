//! Conexion tecnica a Qdrant mediante su cliente oficial.

pub mod connection;
pub mod health;

pub use connection::{QdrantClient, create_connection};
pub use health::check_db_connection;
