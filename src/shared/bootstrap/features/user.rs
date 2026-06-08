use std::sync::Arc;

use crate::features::user::application::user_service::UserService;
use crate::features::user::infrastructure::repositories::user_data_store::UserDataStore;
use crate::features::user::infrastructure::repositories::user_repository::UserRepository;
use crate::features::user::ports::inbound::user_use_case::IUserUseCase;
use crate::shared::bootstrap::error::BootstrapError;
use crate::shared::bootstrap::features::FeatureResources;

pub fn build_user_service(
    resources: FeatureResources,
) -> Result<Arc<dyn IUserUseCase>, BootstrapError> {
    let FeatureResources {
        feature,
        postgres,
        qdrant: _,
        ..
    } = resources;

    let db_pool = postgres.ok_or(FeatureResources::missing_resource(feature, "postgres"))?;
    let user_data_store = UserDataStore::new(db_pool);
    let user_repository = UserRepository::new(user_data_store);

    Ok(Arc::new(UserService::new(user_repository)))
}
