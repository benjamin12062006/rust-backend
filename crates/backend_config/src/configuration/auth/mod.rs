#[derive(Debug, Clone)]
pub enum AuthProviderConfiguration {
    Clerk {
        api_key: String,
    },
    Firebase {
        project_id: String,
        credentials_path: String,
    },
    Local {
        secret: String,
    },
}
