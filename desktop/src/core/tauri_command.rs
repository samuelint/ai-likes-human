use crate::app_state::app_state::AppState;
use app_core::HasProvider;
use app_core::{ConfigurationItemDto, ConfigurationService};
use tauri::State;

#[tauri::command]
pub fn find_configuration(
    app_state: State<AppState>,
    key: String,
) -> Result<Option<ConfigurationItemDto>, String> {
    let mut configuration_service: Box<dyn ConfigurationService> =
        app_state.container.provide().unwrap();

    configuration_service
        .find(key)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn upsert_configuration(
    app_state: State<AppState>,
    key: String,
    value: String,
) -> Result<ConfigurationItemDto, String> {
    let mut configuration_service: Box<dyn ConfigurationService> =
        app_state.container.provide().unwrap();

    configuration_service
        .upsert(key, value)
        .map_err(|err| err.to_string())
}
