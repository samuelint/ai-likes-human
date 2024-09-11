pub use shaku::module;

use crate::{
    app_configuration::AppConfiguration,
    configuration::{
        app::configuration_service::ConfigurationServiceImpl,
        infrastructure::repository::DieselConfigurationRepository,
    },
    infrastructure::database::sqlite_connection_factory::{
        SQliteConnectionFactory, SQliteConnectionFactoryImpl,
    },
};

module! {
    pub AppModule {
        components = [SQliteConnectionFactoryImpl],
        providers = [DieselConfigurationRepository, ConfigurationServiceImpl]
    }
}

pub struct AppContainer {
    pub config: AppConfiguration,
    pub container: AppModule,
}

impl AppContainer {
    pub fn new(config: AppConfiguration) -> Self {
        let container = AppModule::builder()
            .with_component_override::<dyn SQliteConnectionFactory>(Box::new(
                SQliteConnectionFactoryImpl::new(config.database_url.as_str()),
            ))
            .build();

        Self { config, container }
    }
}
