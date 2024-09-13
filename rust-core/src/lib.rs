mod infrastructure;

pub mod app_configuration;
pub mod app_container;
pub mod assistant;
pub mod configuration;
pub mod entities;
pub mod llm;

pub use configuration::app::configuration_service::ConfigurationService;
pub use entities::configuration::*;

pub use shaku::HasProvider; // Required for `app.container.provide()`
