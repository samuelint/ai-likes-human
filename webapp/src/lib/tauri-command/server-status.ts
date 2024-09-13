import { invoke } from '@tauri-apps/api/tauri';

export async function isLocalServerRunning(): Promise<boolean> {
  return invoke('is_server_up');
}