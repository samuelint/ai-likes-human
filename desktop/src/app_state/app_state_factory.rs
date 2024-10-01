use std::fs;

use app::configuration::AppConfigurationBuilder;

use super::app_state::AppState;
use tauri::{AppHandle, Manager};

pub async fn create_and_bind(app_handle: &AppHandle) -> Result<(), std::io::Error> {
    let app_state = create_from_app_handle(app_handle).await?;

    app_handle.manage(app_state);

    Ok(())
}

pub async fn create_from_app_handle(app_handle: &AppHandle) -> Result<AppState, std::io::Error> {
    let data_path = assert_data_path(app_handle)?;

    let app_configuration = AppConfigurationBuilder::new()
        .with_app_data_directory_path(&data_path)
        .with_local_database()
        .with_local_server_port(1234)
        .create();

    Ok(AppState::new(app_configuration).await)
}

fn assert_data_path(app_handle: &AppHandle) -> Result<std::path::PathBuf, std::io::Error> {
    let data_path = app_handle.path_resolver().app_data_dir().unwrap();

    match fs::create_dir_all(&data_path) {
        Ok(_) => Ok(data_path),
        Err(e) => Err(e),
    }
}
