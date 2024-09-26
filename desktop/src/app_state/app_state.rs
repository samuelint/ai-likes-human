use app::configuration::AppConfiguration;
use app_core::{AppContainer, CoreConfiguration};
use std::sync::Arc;

use crate::core::InferenceServer;

pub struct AppState {
    pub configuration: AppConfiguration,
    pub inference_server: Arc<InferenceServer>,
    pub core_container: AppContainer,
}

impl AppState {
    pub async fn new(configuration: AppConfiguration) -> AppState {
        let core_config: CoreConfiguration = (&configuration).into();
        let core_container = AppContainer::new(core_config).await.unwrap();
        let inference_server = Arc::new(InferenceServer::from_configuration(&configuration));

        AppState {
            configuration,
            inference_server,
            core_container,
        }
    }
}
