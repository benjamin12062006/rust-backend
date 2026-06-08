//! Infraestructura compartida para proveedores de datos.
//!
//! Este modulo mantiene los clientes tecnicos fuera de dominio/aplicacion.
//! Cada proveedor vive en su submodulo y luego `bootstrap` decide cuales
//! conexiones crear e inyectar.

pub mod postgres;
pub mod qdrant;
pub mod surrealdb;

pub use postgres::{DbPool, check_db_connection, create_connection};
