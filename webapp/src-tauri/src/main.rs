// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::sync::mpsc::{sync_channel, Receiver};
use std::thread;
use command_group::CommandGroup;
use tauri::api::process::Command as TCommand;
use tauri::WindowEvent;
use tauri_plugin_log::{LogTarget};
use utils::{filesys::local_data_dir_path};

mod utils;

fn start_backend(receiver: Receiver<i32>) {
  // `new_sidecar()` expects just the filename, NOT the whole path
  let t = TCommand::new_sidecar("ai-assistant-core")
    .expect("[Error] Failed to create `ai-assistant-core` binary command");
  let mut group = Command::from(t).group_spawn().expect("[Error] spawning api server process.");
  // If anyone knows how to log out stdout msg's from this process drop a comment. Rust is not my language.
  thread::spawn(move || {
    loop{
      let s = receiver.recv();
      if s.unwrap()==-1 {
        group.kill().expect("[Error] killing api server process.");
      }
    }
  });
}

fn main() {
  let log = tauri_plugin_log::Builder::default().targets([
    LogTarget::Folder(local_data_dir_path()),
    LogTarget::Stdout,
    LogTarget::Webview,
  ])
  .level(log::LevelFilter::Debug);

  let (tx,rx) = sync_channel(1);
  start_backend(rx);

  tauri::Builder::default()
    // Tell the child process to shutdown when app exits
    .on_window_event(move |event| match event.event() {
      WindowEvent::Destroyed => {
        tx.send(-1).expect("[Error] sending msg.");
        println!("[Event] App closed, shutting down API...");
      }
      _ => {}
    })
    .plugin(log.build())
    .run(tauri::generate_context!())
    .expect("[Error] while running tauri application");
}
