pub mod databases;

use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::configuration::configuration::Configuration;

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
