//! Health check minimo para SurrealDB.

use crate::shared::common::error::{CustomError, Result};
use crate::shared::infrastructure::database::surrealdb::SurrealDbClient;

// /// Valida que la conexion SurrealDB responda.
pub async fn check_db_connection(client: &SurrealDbClient) -> Result<()> {
    println!("Checking SurrealDB connection...");
    
    client
        .health()
        .await
        .map_err(|error| CustomError::DatabaseError(error.to_string()))?;
    println!("SurrealDB health check executed successfully!");
    Ok(())
}
