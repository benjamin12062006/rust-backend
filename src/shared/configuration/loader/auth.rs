use std::env;

use crate::shared::configuration::{
    auth::AuthProviderConfiguration, error::config_error::ConfigError,
};

pub fn load_auth_configuration() -> std::result::Result<AuthProviderConfiguration, ConfigError> {
    let provider = env::var("AUTH_PROVIDER").unwrap_or_else(|_| "local".to_string());

    match provider.to_ascii_lowercase().as_str() {
        "clerk" => Ok(AuthProviderConfiguration::Clerk {
            api_key: env::var("CLERK_API_KEY")
                .map_err(|_| ConfigError::MissingEnv("CLERK_API_KEY"))?,
        }),
        "firebase" => Ok(AuthProviderConfiguration::Firebase {
            project_id: env::var("FIREBASE_PROJECT_ID")
                .map_err(|_| ConfigError::MissingEnv("FIREBASE_PROJECT_ID"))?,
            credentials_path: env::var("FIREBASE_CREDENTIALS_PATH")
                .map_err(|_| ConfigError::MissingEnv("FIREBASE_CREDENTIALS_PATH"))?,
        }),
        "local" => Ok(AuthProviderConfiguration::Local {
            secret: env::var("LOCAL_AUTH_SECRET")
                .unwrap_or_else(|_| "local-development-secret".to_string()),
        }),
        _ => Err(ConfigError::InvalidAuthProvider(provider)),
    }
}
