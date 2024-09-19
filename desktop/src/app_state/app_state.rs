use app_core::{AppConfiguration, AppContainer};
use std::{error::Error, sync::Arc};

use crate::core::InferenceServer;

pub struct AppState {
    pub core_server_port: u16,
    inference_server: Arc<InferenceServer>,
    pub core_container: AppContainer,
}

impl AppState {
    pub async fn new(core_server_port: u16) -> AppState {
        let core_container = AppContainer::new(AppConfiguration {
            database_url: "sqlite:///Users/samuel/Desktop/data.db?mode=rwc".to_string(),
        })
        .await
        .unwrap();

        AppState {
            core_server_port,
            inference_server: Arc::new(InferenceServer::new(core_server_port)),
            core_container,
        }
    }

    pub fn is_inference_server_up(&self) -> Result<bool, Box<dyn Error>> {
        self.inference_server.is_core_server_up()
    }

    pub fn start_inference_server(&self) -> Result<(), Box<dyn Error>> {
        let inference_server = Arc::clone(&self.inference_server);
        tauri::async_runtime::spawn(async move {
            inference_server.serve().await;
        });

        Ok(())
    }
}
