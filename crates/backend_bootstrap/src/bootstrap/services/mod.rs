mod service_resources;
mod user;

use backend_http::http_server::Services as ApplicationServices;

use crate::bootstrap::error::BootstrapError;
use crate::bootstrap::resources::AppResources;

pub use service_resources::ServiceResources;

pub fn build_application_services(
    resources: &AppResources,
) -> Result<ApplicationServices, BootstrapError> {
    let user_resources = ServiceResources {
        feature: "user",
        postgres: Some(resources.databases().postgres().clone()),
        surrealdb: None,
        qdrant: None,
    };

    Ok(ApplicationServices {
        user_service: user::build_user_service(user_resources)?,
    })
}
