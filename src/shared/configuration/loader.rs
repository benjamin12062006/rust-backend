use std::env;

use dotenv::dotenv;

use crate::shared::configuration::{
    configuration::{Configuration, ServerConfiguration},
    database::{
        ConfigurationDatabases, PostgresConfiguration, QdrantConfiguration, SurrealDbConfiguration,
    },
    error::config_error::ConfigError,
};

pub fn load_configuration() -> std::result::Result<Configuration, ConfigError> {
    dotenv().ok();

    let postgres = PostgresConfiguration {
        uri: env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnv("DATABASE_URL"))?,
    };

    let surrealdb = SurrealDbConfiguration {
        url: env::var("SURREALDB_URL").map_err(|_| ConfigError::MissingEnv("SURREALDB_URL"))?,
        namespace: env::var("SURREALDB_NAMESPACE")
            .map_err(|_| ConfigError::MissingEnv("SURREALDB_NAMESPACE"))?,
        database: env::var("SURREALDB_DATABASE")
            .map_err(|_| ConfigError::MissingEnv("SURREALDB_DATABASE"))?,
        username: env::var("SURREALDB_USERNAME")
            .map_err(|_| ConfigError::MissingEnv("SURREALDB_USERNAME"))?,
        password: env::var("SURREALDB_PASSWORD")
            .map_err(|_| ConfigError::MissingEnv("SURREALDB_PASSWORD"))?,
    };

    let qdrant_api_key = env::var("QDRANT_API_KEY")
        .ok()
        .filter(|api_key| !api_key.is_empty());

    let qdrant = QdrantConfiguration {
        url: env::var("QDRANT_URL").map_err(|_| ConfigError::MissingEnv("QDRANT_URL"))?,
        api_key: qdrant_api_key,
        collection: env::var("QDRANT_COLLECTION")
            .map_err(|_| ConfigError::MissingEnv("QDRANT_COLLECTION"))?,
    };

    let server = ServerConfiguration {
        port: env::var("PORT").map_err(|_| ConfigError::MissingEnv("PORT"))?,
    };

    let databases = ConfigurationDatabases::new(postgres, surrealdb, qdrant);

    Ok(Configuration::new(databases, server))
}
