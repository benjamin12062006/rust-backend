//! Health check minimo para Qdrant.

use crate::shared::common::error::Result;
use crate::shared::infrastructure::database::qdrant::QdrantClient;

/// Valida que el servidor Qdrant responda por gRPC.
pub async fn check_db_connection(client: &QdrantClient)->Result<()> {
    println!("Checking Qdrant connection...");
    
    client.health_check().await?;
    println!("Qdrant health check executed successfully!");
    Ok(())
}




















// pub async fn check_db_connection(client: &QdrantClient) -> Result<()> {
//     println!("Checking Qdrant connection...");

//     client.health_check().await?;
//     println!("Qdrant health check executed successfully!");
//     Ok(())
// }
