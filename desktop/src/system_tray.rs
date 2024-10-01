use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
static QUIT_APP_LABEL: &'static str = "quit";
static SHOW_MAIN_WINDOW_APP_LABEL: &'static str = "show";

pub fn build_menu() -> SystemTrayMenu {
    let menuitem_quit = CustomMenuItem::new(QUIT_APP_LABEL, "Quit AI Likes Human");
    let menuitem_show = CustomMenuItem::new(SHOW_MAIN_WINDOW_APP_LABEL, "Show Main Window");
    SystemTrayMenu::new()
        .add_item(menuitem_show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(menuitem_quit)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id {
            id if id == QUIT_APP_LABEL => app.exit(0),
            id if id == SHOW_MAIN_WINDOW_APP_LABEL => {
                app.get_window("main").unwrap().show().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
