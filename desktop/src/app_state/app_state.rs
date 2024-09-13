use app_core::openai_server_api;
use app_core::{AppConfiguration, AppContainer, AppModule};
use std::error::Error;

pub struct AppState {
    pub core_server_port: u16,
    pub container: AppModule,
}

impl AppState {
    pub async fn new(core_server_port: u16) -> AppState {
        let app = AppContainer::create(AppConfiguration {
            database_url: "sqlite:///Users/samuel/Desktop/data.db?mode=rwc".to_string(),
        })
        .await
        .unwrap();

        AppState {
            core_server_port,
            container: app.container,
        }
    }

    pub fn is_core_server_up(&self) -> Result<bool, Box<dyn Error>> {
        let response = ureq::get(&format!("http://localhost:{}", self.core_server_port)).call()?;
        let status = response.status();

        return Ok(status == 200);
    }

    pub fn start_core_server(&self) -> Result<(), Box<dyn Error>> {
        tauri::async_runtime::spawn(openai_server_api::serve(
            openai_server_api::ServeParameters {
                port: self.core_server_port,
            },
        ));

        Ok(())
    }
}
