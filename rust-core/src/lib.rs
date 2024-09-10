mod app_configuration;
mod app_container;
mod configuration;
mod infrastructure;
mod schema;

use std::error::Error;

use app_configuration::AppConfiguration;
use app_container::AppContainer;
use configuration::domain::{
    configuration_service::Service, dto::NewConfigurationItemDto,
    repository::ConfigurationRepository,
};
use shaku::HasProvider;

pub fn create_app(config: AppConfiguration) -> AppContainer {
    AppContainer::create(config)
}

pub fn start() -> Result<(), Box<dyn Error>> {
    let app = create_app(AppConfiguration {
        database_url: "sqlite:///Users/samuel/Desktop/data.db".to_string(),
    });
    let mut config_repo: Box<dyn ConfigurationRepository> = app.container.provide().unwrap();
    let mut service: Box<dyn Service> = app.container.provide().unwrap();

    let model = config_repo.create(NewConfigurationItemDto {
        key: "key2",
        value: "value22",
    })?;
    println!("Item created! {}: {}", model.key, model.value);

    let existing = config_repo.get_all()?;
    println!("All existing items from repo {:#?}", existing);

    let existing = service.get_all()?;
    println!("All existing items from service {:#?}", existing);

    Ok(())
}
