use std::sync::Arc;

use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::configuration::database::QdrantConfiguration;
use crate::shared::infrastructure::database::qdrant::{self, QdrantClient};

pub async fn initialize(config: &QdrantConfiguration) -> Result<Arc<QdrantClient>, BootstrapError> {
    let client = qdrant::create_connection(config)?;
    qdrant::check_db_connection(&client).await?;

    Ok(Arc::new(client))
}
