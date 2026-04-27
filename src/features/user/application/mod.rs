//! Slice de application para el subdominio de usuario.
//!
//! Idea central:
//! - `use_cases` concentra la logica del negocio de application.
//! - `user_service` funciona como fachada del puerto de entrada y delega.
//! - `dto` define contratos de entrada/salida.
//! - `policies` encapsula reglas auxiliares (ej: hash de password).

pub mod dto;
pub mod policies;
pub mod use_cases;
pub mod user_service;
