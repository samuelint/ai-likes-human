use shaku::module;

use crate::{
    app_configuration::AppConfiguration,
    configuration::{
        domain::configuration_service::ServiceImpl,
        infrastructure::repository::DieselConfigurationRepository,
    },
    infrastructure::database::sqlite_connection_factory::{
        SQliteConnectionFactory, SQliteConnectionFactoryImpl,
    },
};

module! {
    pub AppModule {
        components = [SQliteConnectionFactoryImpl],
        providers = [DieselConfigurationRepository, ServiceImpl]
    }
}

pub struct AppContainer {
    pub config: AppConfiguration,
    pub container: AppModule,
}

impl AppContainer {
    pub fn create(config: AppConfiguration) -> Self {
        let container = AppModule::builder()
            .with_component_override::<dyn SQliteConnectionFactory>(Box::new(
                SQliteConnectionFactoryImpl::new(config.database_url.as_str()),
            ))
            .build();

        Self { config, container }
    }
}
