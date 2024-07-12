// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app_state;
pub mod screencapture;
pub mod system_tray;

use tauri_plugin_log::LogTarget;
use tauri::{Manager, State, WindowEvent, SystemTray};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use screencapture::capture_screen;
use system_tray::{build_menu, on_system_tray_event};
use app_state::app_state::{AppState, start_server, stop_server, is_server_up};
use log::info;



fn main() {
  let tray_menu = build_menu();

  let state = AppState::new();

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
    .setup(move |app| {
      let app_state: State<AppState> = app.state();

      let is_up = app_state.is_core_server_up();
      if !is_up {
        app_state.start_core_server().expect("Core Sidecar start failed");
      }

      Ok(())
    })
    .on_window_event(move |event| match event.event() {
      WindowEvent::CloseRequested { api, .. } => {
        info!("Window CloseRequested");
        #[cfg(not(target_os = "macos"))] {
          event.window().hide().unwrap();
        }

        #[cfg(target_os = "macos")] {
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
      start_server,
      stop_server,
      is_server_up,
      capture_screen,
    ])
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|app_handle, event| match event {
      tauri::RunEvent::Exit => {
        let app_state: State<AppState> = app_handle.state();
        app_state.stop_core_server().expect("Core Sidecar stop failed");
      }
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    });
}
