//! Health check minimo para Postgres.

use crate::shared::common::error::Result;
use crate::shared::infrastructure::database::postgres::DbPool;

/// Ejecuta un ping SQL minimo para validar conectividad con la base.
pub async fn check_db_connection(pool: &DbPool) -> Result<()> {
    println!("Checking Postgres connection...");

    sqlx::query("SELECT 1").fetch_one(pool).await?;
    println!("Postgres PING executed successfully!");
    Ok(())
}
