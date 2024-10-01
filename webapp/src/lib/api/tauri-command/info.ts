import { invoke } from '@tauri-apps/api/tauri';

export async function getAppDirectoryPath(): Promise<string> {
  return invoke('get_app_directory_path');
}