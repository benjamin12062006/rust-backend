//! Infraestructura del subdominio `user`.
//!
//! `user_data_store`: SQL concreto.
//! `user_entity`: modelo tecnico de fila.
//! `user_repository`: adapter que implementa el puerto `IUserRepository`.

pub mod user_data_store;
pub mod user_entity;
pub mod user_repository;
