// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  api::process::{Command, CommandEvent},
  Manager,
};

fn log_handler(window: &tauri::Window, event: &CommandEvent) {
  if let CommandEvent::Stderr(error_line) = event {
    eprintln!("{error_line}");
    window
      .emit("local-server-stderr", Some(format!("'{}'", error_line)))
      .expect("failed to emit event");
  }
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window: tauri::Window = app.get_window("main").unwrap();
      tauri::async_runtime::spawn(async move {
        let (mut rx, mut _child) = Command::new_sidecar("ai-assistant-core")
          .expect("failed to setup `ai-assistant-core` sidecar")
          .spawn()
          .expect("Failed to spawn packaged node");

        while let Some(event) = rx.recv().await {
          log_handler(&window, &event)
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("[Error] while running tauri application");
}
