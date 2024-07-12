use std::sync::Mutex;
use tauri::State;
use super::sidecar_lifecycle_service::SidecarLifeCycleService;

pub struct AppState {
    code_sidecar_mutex: Mutex<SidecarLifeCycleService>,
}

impl AppState {
  pub fn new() -> AppState {
    let core_sidecar = SidecarLifeCycleService::new("core");
    AppState {
      code_sidecar_mutex: Mutex::new(core_sidecar),
    }
  }

  pub fn start(&self) -> Result<String, String> {
    return self.code_sidecar_mutex
      .lock()
      .unwrap()
      .start();
  }

  pub fn stop(&self) -> Result<String, String> {
    return self.code_sidecar_mutex
      .lock()
      .unwrap()
      .stop();
  }
}

#[tauri::command]
pub fn start_server(api_manager_state: State<AppState>) -> Result<String, String> {
  let am = api_manager_state
      .code_sidecar_mutex
      .lock()
      .unwrap()
      .start();
  am
}

#[tauri::command]
pub fn stop_server(api_manager_state: State<AppState>) -> Result<String, String> {
  let app_state = api_manager_state
      .code_sidecar_mutex
      .lock()
      .unwrap()
      .stop();
  app_state
}
