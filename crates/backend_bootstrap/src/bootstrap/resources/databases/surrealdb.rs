use std::sync::Arc;

use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::configuration::database::SurrealDbConfiguration;
use crate::shared::infrastructure::database::surrealdb::{self, SurrealDbClient};

pub async fn initialize(
    config: &SurrealDbConfiguration,
) -> Result<Arc<SurrealDbClient>, BootstrapError> {
    let client = surrealdb::create_connection(config).await?;
    surrealdb::check_db_connection(&client).await?;

    Ok(Arc::new(client))
}
