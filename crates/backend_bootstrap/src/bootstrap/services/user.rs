use std::sync::Arc;

use backend_application::user_application::user_service::UserService;
use backend_infrastructure::user_infrastructure::repositories::user_data_store::UserDataStore;
use backend_infrastructure::user_infrastructure::repositories::user_repository::UserRepository;
use backend_application::user_ports::inbound::user_use_case::IUserUseCase;
use crate::bootstrap::error::BootstrapError;
use crate::bootstrap::services::ServiceResources;

pub fn build_user_service(
    resources: ServiceResources,
) -> Result<Arc<dyn IUserUseCase>, BootstrapError> {
    let ServiceResources {
        feature,
        postgres,
        qdrant: _,
        ..
    } = resources;

    let db_pool = postgres.ok_or(ServiceResources::missing_resource(feature, "postgres"))?;
    let user_data_store = UserDataStore::new(db_pool);
    let user_repository = UserRepository::new(user_data_store);

    Ok(Arc::new(UserService::new(user_repository)))
}
