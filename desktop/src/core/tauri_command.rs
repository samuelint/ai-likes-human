use crate::app_state::app_state::AppState;
use app_core::entities::configuration;
use tauri::State;

#[tauri::command]
pub fn is_server_up(app_state: State<'_, AppState>) -> Result<bool, String> {
    app_state
        .inference_server
        .is_up()
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get_inference_server_url(app_state: State<'_, AppState>) -> String {
    app_state.inference_server.get_url()
}

#[tauri::command]
pub async fn find_configuration(
    app_state: State<'_, AppState>,
    key: String,
) -> Result<Option<configuration::Model>, String> {
    let configuration_service = app_state
        .core_container
        .configuration_module
        .get_configuration_service();

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
    let configuration_service = app_state
        .core_container
        .configuration_module
        .get_configuration_service();

    configuration_service
        .upsert(key, value)
        .await
        .map_err(|err| err.to_string())
}
