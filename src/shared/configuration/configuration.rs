use crate::shared::configuration::auth::AuthProviderConfiguration;
use crate::shared::configuration::database::ConfigurationDatabases;

pub struct ServerConfiguration {
    pub port: String,
}

pub struct Configuration {
    pub auth: AuthProviderConfiguration,
    pub databases: ConfigurationDatabases,
    pub server: ServerConfiguration,
}

impl Configuration {
    pub fn new(
        auth: AuthProviderConfiguration,
        databases: ConfigurationDatabases,
        server: ServerConfiguration,
    ) -> Self {
        Self {
            auth,
            databases,
            server,
        }
    }
}
