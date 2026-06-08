//! Construccion del cliente Qdrant.
//!
//! El cliente oficial se conecta por gRPC; en Docker local usamos
//! `http://localhost:6334`.

use qdrant_client::Qdrant;

use crate::shared::common::error::{CustomError, Result};
use crate::shared::configuration::database::QdrantConfiguration;


pub type QdrantClient = Qdrant;
// /// Crea un cliente Qdrant desde URL y API key opcional.
pub fn create_connection(config: &QdrantConfiguration)-> Result<QdrantClient> {
    let builder = Qdrant::from_url(config.url.as_str());
    let builder = match config.api_key.as_deref() {
        Some(api_key) if !api_key.is_empty() => builder.api_key(api_key),
        _ => builder
    };
    
    builder
        .build()
        .map_err(|error| CustomError::DatabaseError(error.to_string()))
}
