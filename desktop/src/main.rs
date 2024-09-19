// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app_state;
pub mod core;
pub mod screencapture;
pub mod system_tray;

use app_state::app_state::AppState;
use core::tauri_command::{find_configuration, is_server_up, upsert_configuration};
use log::info;
use screencapture::tauri_command::{assert_screen_capture_permissions, capture_screen};
use system_tray::{build_menu, on_system_tray_event};
use tauri::{Manager, State, SystemTray, WindowEvent};
use tauri_plugin_log::LogTarget;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

static CORE_SERVER_PORT_NUMBER: u16 = 1234;

#[tokio::main]
async fn main() {
    let tray_menu = build_menu();

    let state = AppState::new(CORE_SERVER_PORT_NUMBER).await;

    let log_builder = tauri_plugin_log::Builder::default().targets([
        LogTarget::LogDir,
        LogTarget::Stdout,
        LogTarget::Stderr,
        LogTarget::Webview,
    ]);

    tauri::Builder::default()
        .manage(state)
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(on_system_tray_event)
        .plugin(log_builder.build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let app_state: State<AppState> = app.state();

            app_state.start_inference_server()
        })
        .on_window_event(move |event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                info!("Window CloseRequested");
                #[cfg(not(target_os = "macos"))]
                {
                    event.window().hide().unwrap();
                }

                #[cfg(target_os = "macos")]
                {
                    tauri::AppHandle::hide(&event.window().app_handle()).unwrap();
                }
                api.prevent_close();
            }
            WindowEvent::Destroyed => {
                info!("Window destroyed");

                let app_handle: tauri::AppHandle = event.window().app_handle();
                app_handle.save_window_state(StateFlags::all()).ok();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            is_server_up,
            capture_screen,
            assert_screen_capture_permissions,
            upsert_configuration,
            find_configuration,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
