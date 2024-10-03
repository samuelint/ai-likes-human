use std::sync::Arc;

use domain::{ProfileRepository, ProfileSystemPromptFactory, SelectedProfileService};
use infrastructure::SeaOrmProfileRepository;

pub mod domain;
pub mod infrastructure;

pub struct ProfileDIModule {
    connection: Arc<::sea_orm::DatabaseConnection>,
}

impl ProfileDIModule {
    pub fn new(connection: Arc<::sea_orm::DatabaseConnection>) -> Self {
        Self { connection }
    }

    fn get_connection(&self) -> Arc<::sea_orm::DatabaseConnection> {
        Arc::clone(&self.connection)
    }

    pub fn get_profile_repository(&self) -> Arc<dyn ProfileRepository> {
        let connection = self.get_connection();
        Arc::new(SeaOrmProfileRepository::new(Arc::clone(&connection)))
    }

    pub fn get_selected_profiles_service(&self) -> Arc<SelectedProfileService> {
        let profile_repository = self.get_profile_repository();
        Arc::new(SelectedProfileService::new(Arc::clone(&profile_repository)))
    }

    pub fn get_profile_system_prompt_factory(&self) -> Arc<ProfileSystemPromptFactory> {
        let selected_profile_service = self.get_selected_profiles_service();

        Arc::new(ProfileSystemPromptFactory::new(
            selected_profile_service.clone(),
        ))
    }
}
