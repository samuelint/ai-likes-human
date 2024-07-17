import { invoke } from '@tauri-apps/api/tauri';


export async function startLocalServer(): Promise<void> {
  return invoke('start_server');
}

export async function stopLocalServer(): Promise<void> {
  return invoke('stop_server');
}

export async function isLocalServerRunning(): Promise<boolean> {
  return invoke('is_server_up');
}