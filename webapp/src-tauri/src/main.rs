// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod sidecar_lifecycle_service;

use std::sync::Mutex;
use tauri_plugin_log::LogTarget;
use tauri::{Manager, State, WindowEvent};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use sidecar_lifecycle_service::SidecarLifeCycleService;

struct AppState {
  code_sidecar_mutex: Mutex<SidecarLifeCycleService>,
}

#[tauri::command]
fn start_server(api_manager_state: State<AppState>) -> Result<String, String> {
  let am = api_manager_state
      .code_sidecar_mutex
      .lock()
      .unwrap()
      .start();
  am
}

#[tauri::command]
fn stop_server(api_manager_state: State<AppState>) -> Result<String, String> {
  let app_state = api_manager_state
      .code_sidecar_mutex
      .lock()
      .unwrap()
      .stop();
  app_state
}


fn main() {
  let core_sidecar = SidecarLifeCycleService::new("core");
  let state = AppState {
      code_sidecar_mutex: Mutex::new(core_sidecar),
  };

  let log_builder = tauri_plugin_log::Builder::default().targets([
    LogTarget::LogDir,
    LogTarget::Stdout,
    LogTarget::Stderr,
    LogTarget::Webview,
  ]);

  tauri::Builder::default()
    .manage(state)
    .plugin(log_builder.build())  
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(move |app| {
        let app_state: State<AppState> = app.state();
        app_state.code_sidecar_mutex
            .lock()
            .unwrap()
            .start()
            .expect("Core Sidecar start failed");
        Ok(())
    })
    .on_window_event(move |event| match event.event() {
        WindowEvent::Destroyed => {
            let am: State<AppState> = event.window().state();
            am.code_sidecar_mutex
                .lock()
                .unwrap()
                .stop()
                .expect("Core Sidecar stop failed");

            let app_handle: tauri::AppHandle = event.window().app_handle();
            app_handle.save_window_state(StateFlags::all()).ok();
        }
        _ => {}
    })
    .invoke_handler(tauri::generate_handler![
        start_server,
        stop_server,
    ])
    .run(tauri::generate_context!())
    .expect("[Error] while running tauri application");
}
