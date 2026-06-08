mod feature_resources;
mod user;

use backend_http::http_server::Services as ApplicationServices;

use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::bootstrap::resources::AppResources;

pub use feature_resources::FeatureResources;

pub fn build_application_services(
    resources: &AppResources,
) -> Result<ApplicationServices, BootstrapError> {
    let user_resources = FeatureResources {
        feature: "user",
        postgres: Some(resources.databases().postgres().clone()),
        surrealdb: None,
        qdrant: None,
    };

    Ok(ApplicationServices {
        user_service: user::build_user_service(user_resources)?,
    })
}
