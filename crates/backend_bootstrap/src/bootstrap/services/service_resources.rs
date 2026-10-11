use std::sync::Arc;

use crate::bootstrap::error::BootstrapError;
use backend_infrastructure::database::postgres::DbPool;
use backend_infrastructure::database::qdrant::QdrantClient;
use backend_infrastructure::database::surrealdb::SurrealDbClient;

#[derive(Clone)]
pub struct ServiceResources {
    pub feature: &'static str,
    pub postgres: Option<DbPool>,
    pub surrealdb: Option<Arc<SurrealDbClient>>,
    pub qdrant: Option<Arc<QdrantClient>>,
}

impl ServiceResources {
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
