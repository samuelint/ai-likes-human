use std::sync::Arc;

use app::{ConfigurationService, ConfigurationServiceImpl};
use domain::ConfigurationRepository;
use infrastructure::SeaOrmConfigurationRepository;

pub mod app;
pub mod domain;
pub mod infrastructure;
pub use domain::dto::*;

pub struct ConfigurationDIModule {
    connection: Arc<::sea_orm::DatabaseConnection>,
}

impl ConfigurationDIModule {
    pub fn new(connection: Arc<::sea_orm::DatabaseConnection>) -> Self {
        Self { connection }
    }

    fn get_connection(&self) -> Arc<::sea_orm::DatabaseConnection> {
        Arc::clone(&self.connection)
    }

    pub fn get_configuration_repository(&self) -> Arc<dyn ConfigurationRepository> {
        let connection = self.get_connection();
        Arc::new(SeaOrmConfigurationRepository::new(Arc::clone(&connection)))
    }

    pub fn get_configuration_service(&self) -> Arc<dyn ConfigurationService> {
        let configuration_repository = self.get_configuration_repository();

        Arc::new(ConfigurationServiceImpl::new(Arc::clone(
            &configuration_repository,
        )))
    }
}
