pub mod databases;

use crate::bootstrap::error::BootstrapError;
use backend_config::configuration::configuration::Configuration;

pub use databases::DatabaseResources;

pub struct AppResources {
    databases: DatabaseResources,
}

impl AppResources {
    pub async fn initialize(config: &Configuration) -> Result<Self, BootstrapError> {
        let databases = DatabaseResources::initialize(&config.databases).await?;

        Ok(Self { databases })
    }

    pub fn databases(&self) -> &DatabaseResources {
        &self.databases
    }
}
