use crate::shared::configuration::database::ConfigurationDatabases;

pub struct ServerConfiguration {
    pub port: String,
}

pub struct Configuration {
    pub databases: ConfigurationDatabases,
    pub server: ServerConfiguration,
}

impl Configuration {
    pub fn new(databases: ConfigurationDatabases, server: ServerConfiguration) -> Self {
        Self {
            databases,
            server,
        }
    }
}
