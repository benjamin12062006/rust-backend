use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::bootstrap::features::build_application_services;
use crate::shared::bootstrap::http_server::Server;
use crate::shared::bootstrap::resources::AppResources;
use crate::shared::configuration::loader::load_configuration;

pub async fn run() -> Result<(), BootstrapError> {
    let config = load_configuration()?;
    let resources = AppResources::initialize(&config).await?;
    let services = build_application_services(&resources)?;

    let port = config
        .server
        .port
        .parse::<u16>()
        .map_err(|_| BootstrapError::InvalidPort(config.server.port.clone()))?;

    let server = Server::new(port, services);
    server.run().await;
    Ok(())
}
