use sqlx::migrate::Migrator;

use crate::bootstrap::error::BootstrapError;
use backend_config::configuration::database::PostgresConfiguration;
use backend_infrastructure::database::postgres::{self, DbPool};

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

pub async fn initialize(config: &PostgresConfiguration) -> Result<DbPool, BootstrapError> {
    let pool = postgres::create_connection(&config.uri).await?;
    postgres::check_db_connection(&pool).await?;
    MIGRATOR
        .run(&pool)
        .await
        .map_err(|error| BootstrapError::Migration(error.to_string()))?;

    Ok(pool)
}
