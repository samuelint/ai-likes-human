use super::sidecar_lifecycle_service::SidecarLifeCycleService;
use app_core::{
    app_configuration::AppConfiguration,
    app_container::{AppContainer, AppModule},
};
use log::info;
use std::sync::Mutex;
use tauri::State;
use ureq;

pub struct AppState {
    core_server_port: u16,
    code_sidecar_mutex: Mutex<SidecarLifeCycleService>,
    pub container: AppModule,
}

impl AppState {
    pub fn new(port: u16) -> AppState {
        let core_sidecar = SidecarLifeCycleService::new("core");
        let app = AppContainer::new(AppConfiguration {
            database_url: "sqlite:///Users/samuel/Desktop/data.db".to_string(),
        });

        AppState {
            core_server_port: port,
            code_sidecar_mutex: Mutex::new(core_sidecar),
            container: app.container,
        }
    }

    pub fn start_core_server(&self) -> Result<String, String> {
        return self.code_sidecar_mutex.lock().unwrap().start();
    }

    pub fn stop_core_server(&self) -> Result<String, String> {
        return self.code_sidecar_mutex.lock().unwrap().stop();
    }

    pub fn is_core_server_up(&self) -> bool {
        let response =
            match ureq::get(&format!("http://localhost:{}", self.core_server_port)).call() {
                Ok(res) => res,
                Err(err) => {
                    info!("Failed to query the server: {}", err);
                    return false;
                }
            };

        let status = response.status();

        return status == 200;
    }

    pub fn wait_core_server_to_be_up_for(&self, delay_sec: u32) -> bool {
        let mut elapsed = 0;
        loop {
            let is_up = self.is_core_server_up();

            if is_up {
                return true;
            }

            elapsed += 1;
            if elapsed >= delay_sec {
                return false;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

#[tauri::command]
pub fn start_server(api_manager_state: State<AppState>) -> Result<String, String> {
    api_manager_state.start_core_server()
}

#[tauri::command]
pub fn stop_server(api_manager_state: State<AppState>) -> Result<String, String> {
    api_manager_state.stop_core_server()
}

#[tauri::command]
pub fn is_server_up(api_manager_state: State<AppState>) -> Result<bool, String> {
    let is_up = api_manager_state.is_core_server_up();

    Ok(is_up)
}
