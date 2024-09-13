mod app_configuration;
mod app_container;
mod assistant;
mod configuration;
mod infrastructure;
mod llm;

pub mod entities;

pub use app_configuration::AppConfiguration;
pub use app_container::{AppContainer, AppModule};
pub use configuration::app::*;
pub use entities::configuration::*;

pub use openai_server_api;
pub use shaku::HasProvider; // Required for `app.container.provide()`
