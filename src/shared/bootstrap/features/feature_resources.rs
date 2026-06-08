use std::sync::Arc;

use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::infrastructure::database::postgres::DbPool;
use crate::shared::infrastructure::database::qdrant::QdrantClient;
use crate::shared::infrastructure::database::surrealdb::SurrealDbClient;

#[derive(Clone)]
pub struct FeatureResources {
    pub feature: &'static str,
    pub postgres: Option<DbPool>,
    pub surrealdb: Option<Arc<SurrealDbClient>>,
    pub qdrant: Option<Arc<QdrantClient>>,
}

impl FeatureResources {
    pub fn new(feature: &'static str) -> Self {
        Self {
            feature,
            postgres: None,
            surrealdb: None,
            qdrant: None,
        }
    }

    pub fn missing_resource(feature: &'static str, resource: &'static str) -> BootstrapError {
        BootstrapError::MissingFeatureResource { feature, resource }
    }
}
