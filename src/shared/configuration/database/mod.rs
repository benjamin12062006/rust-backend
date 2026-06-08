//! Configuracion agrupada de proveedores de base de datos.
//!
//! Este modulo concentra solo settings de persistencia para que la raiz
//! `Configuration` no crezca desordenada a medida que agregamos proveedores.

#[derive(Debug, Clone)]
pub struct PostgresConfiguration {
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct SurrealDbConfiguration {
    pub url: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct QdrantConfiguration {
    pub url: String,
    pub api_key: Option<String>,
    pub collection: String,
}

#[derive(Debug, Clone)]
pub struct ConfigurationDatabases {
    pub postgres: PostgresConfiguration,
    pub surrealdb: SurrealDbConfiguration,
    pub qdrant: QdrantConfiguration,
}

impl ConfigurationDatabases {
    pub fn new(
        postgres: PostgresConfiguration,
        surrealdb: SurrealDbConfiguration,
        qdrant: QdrantConfiguration,
    ) -> Self {
        Self {
            postgres,
            surrealdb,
            qdrant,
        }
    }
}
