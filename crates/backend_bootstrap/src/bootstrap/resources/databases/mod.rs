mod postgres;
mod qdrant;
mod surrealdb;

use std::sync::Arc;

use crate::bootstrap::error::BootstrapError;
use backend_config::configuration::database::ConfigurationDatabases;
use backend_infrastructure::database::postgres::DbPool;
use backend_infrastructure::database::qdrant::QdrantClient;
use backend_infrastructure::database::surrealdb::SurrealDbClient;

pub struct DatabaseResources {
    postgres: DbPool,
    surrealdb: Arc<SurrealDbClient>,
    qdrant: Arc<QdrantClient>,
}

impl DatabaseResources {
    pub async fn initialize(config: &ConfigurationDatabases) -> Result<Self, BootstrapError> {
        let postgres = postgres::initialize(&config.postgres).await?;
        let surrealdb = surrealdb::initialize(&config.surrealdb).await?;
        let qdrant = qdrant::initialize(&config.qdrant).await?;

        Ok(Self {
            postgres,
            surrealdb,
            qdrant,
        })
    }

    pub fn postgres(&self) -> &DbPool {
        &self.postgres
    }

    pub fn surrealdb(&self) -> &Arc<SurrealDbClient> {
        &self.surrealdb
    }

    pub fn qdrant(&self) -> &Arc<QdrantClient> {
        &self.qdrant
    }
}
