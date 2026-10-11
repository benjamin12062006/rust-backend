//! Construccion del pool de conexiones Postgres.

use sqlx::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};

use backend_common::common::error::{CustomError, Result};

const DB_POOL_MAX_CONNECTIONS: u32 = 5;

pub type DbPool = Pool<Postgres>;

/// Crea el pool de conexiones a Postgres a partir del `db_uri` configurado.
pub async fn create_connection(db_uri: &str) -> Result<DbPool> {
    PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_CONNECTIONS)
        .connect(db_uri)
        .await
        .map_err(|error| CustomError::DatabaseError(error.to_string()))
}
