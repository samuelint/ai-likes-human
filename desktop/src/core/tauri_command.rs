use crate::app_state::app_state::AppState;
use app_core::HasProvider;
use app_core::{entities::configuration, ConfigurationService};
use tauri::State;

#[tauri::command]
pub fn is_server_up(app_state: State<'_, AppState>) -> Result<bool, String> {
    app_state.is_core_server_up().map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn find_configuration(
    app_state: State<'_, AppState>,
    key: String,
) -> Result<Option<configuration::Model>, String> {
    let configuration_service: Box<dyn ConfigurationService> =
        app_state.container.provide().unwrap();

    let r = configuration_service
        .find(key)
        .await
        .map_err(|err| err.to_string());

    r
}

#[tauri::command]
pub async fn upsert_configuration(
    app_state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<configuration::Model, String> {
    let configuration_service: Box<dyn ConfigurationService> =
        app_state.container.provide().unwrap();

    configuration_service
        .upsert(key, value)
        .await
        .map_err(|err| err.to_string())
}
