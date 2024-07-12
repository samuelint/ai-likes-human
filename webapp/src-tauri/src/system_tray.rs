use super::app_state::app_state::AppState;
use log::info;
use tauri::{State, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, AppHandle, Manager};

pub fn build_menu() -> SystemTrayMenu {
    let menuitem_quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let menuitem_show = CustomMenuItem::new("show".to_string(), "Show");
    SystemTrayMenu::new()
        .add_item(menuitem_show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(menuitem_quit)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                let window = app.get_window("main").unwrap();
                let app_state: State<AppState> = window.state();
                app_state.stop().expect("Core Sidecar stop failed");
                app.exit(0)
            }
            "show" => {
                info!("Show");
                app.get_window("main").unwrap().show().unwrap();
            }
            _ => {}
        }
        _ => {}
    }
}
