use std::env;

use dotenv::dotenv;

use crate::shared::configuration::{
    configuration::{Configuration, ServerConfiguration},
    error::config_error::ConfigError,
};

mod auth;
mod databases;

pub fn load_configuration() -> std::result::Result<Configuration, ConfigError> {
    dotenv().ok();

    let auth = auth::load_auth_configuration()?;
    let databases = databases::load_databases_configuration()?;
    let server = load_server_configuration()?;

    Ok(Configuration::new(auth, databases, server))
}

fn load_server_configuration() -> std::result::Result<ServerConfiguration, ConfigError> {
    Ok(ServerConfiguration {
        port: env::var("PORT").map_err(|_| ConfigError::MissingEnv("PORT"))?,
    })
}
