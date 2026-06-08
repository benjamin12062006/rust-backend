//! Puertos de salida (driven ports).
//!
//! Definen dependencias externas que application necesita para ejecutar casos
//! de uso (repositorios, servicios, etc).

pub mod user;
pub use user::user_repository;
