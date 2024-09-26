use crate::app_state::app_state::AppState;
use app_core::configuration::ConfigurationDto;
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
pub fn get_app_directory_path(app_state: State<'_, AppState>) -> String {
    app_state.configuration.app_data_directory_path.clone()
}

#[tauri::command]
pub async fn find_configuration(
    app_state: State<'_, AppState>,
    key: &str,
) -> Result<Option<ConfigurationDto>, String> {
    app_state
        .api
        .find_configuration(key)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn upsert_configuration(
    app_state: State<'_, AppState>,
    key: &str,
    value: &str,
) -> Result<ConfigurationDto, String> {
    app_state
        .api
        .upsert_configuration(key, value)
        .await
        .map_err(|err| err.to_string())
}
