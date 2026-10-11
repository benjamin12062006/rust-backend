use std::sync::Arc;

use crate::bootstrap::error::BootstrapError;
use backend_config::configuration::database::SurrealDbConfiguration;
use backend_infrastructure::database::surrealdb::{self, SurrealDbClient};

pub async fn initialize(
    config: &SurrealDbConfiguration,
) -> Result<Arc<SurrealDbClient>, BootstrapError> {
    let client = surrealdb::create_connection(config).await?;
    surrealdb::check_db_connection(&client).await?;

    Ok(Arc::new(client))
}
