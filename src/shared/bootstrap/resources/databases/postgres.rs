use sqlx::migrate::Migrator;

use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::configuration::database::PostgresConfiguration;
use crate::shared::infrastructure::database::postgres::{self, DbPool};

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn initialize(config: &PostgresConfiguration) -> Result<DbPool, BootstrapError> {
    let pool = postgres::create_connection(&config.uri).await?;
    postgres::check_db_connection(&pool).await?;
    MIGRATOR
        .run(&pool)
        .await
        .map_err(|error| BootstrapError::Migration(error.to_string()))?;

    Ok(pool)
}
