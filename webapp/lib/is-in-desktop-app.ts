export function isInDesktopApp() {
  return window.__TAURI_IPC__ != null;
}