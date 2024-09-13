use std::error::Error;

pub use shaku::module;

use crate::{
    app_configuration::AppConfiguration,
    configuration::{
        app::configuration_service::ConfigurationServiceImpl,
        infrastructure::sea_orm_repository::SeaOrmConfigurationRepository,
    },
    infrastructure::sea_orm,
};

module! {
    pub AppModule {
        components = [sea_orm::ConnectionProviderImpl],
        providers = [SeaOrmConfigurationRepository, ConfigurationServiceImpl]
    }
}

pub struct AppContainer {
    pub config: AppConfiguration,
    pub container: AppModule,
}

impl AppContainer {
    pub async fn create(config: AppConfiguration) -> Result<Self, Box<dyn Error>> {
        let connection_factory = sea_orm::ConnectionFactory::new(config.database_url.clone());
        let connection = connection_factory.create().await?;

        let container = AppModule::builder()
            .with_component_parameters::<sea_orm::ConnectionProviderImpl>(
                sea_orm::ConnectionProviderImplParameters { connection },
            )
            .build();

        Ok(Self { config, container })
    }
}
