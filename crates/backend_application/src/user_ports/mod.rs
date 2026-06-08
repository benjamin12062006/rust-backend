//! Puertos de la arquitectura hexagonal.
//!
//! `inbound`: contratos de entrada hacia application.
//! `out`: contratos de salida que application necesita del exterior.

#[path = "in/mod.rs"]
pub mod inbound;
pub mod out; // r#:in sirve para evitar conflictos con palabras reservadas de Rust
