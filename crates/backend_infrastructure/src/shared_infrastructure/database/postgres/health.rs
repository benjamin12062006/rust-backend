//! Health check minimo para Postgres.

use crate::shared::common::error::{CustomError, Result};
use crate::shared::infrastructure::database::postgres::DbPool;

/// Ejecuta un ping SQL minimo para validar conectividad con la base.
pub async fn check_db_connection(pool: &DbPool) -> Result<()> {
    println!("Checking Postgres connection...");

    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|error| CustomError::DatabaseError(error.to_string()))?;
    println!("Postgres PING executed successfully!");
    Ok(())
}
