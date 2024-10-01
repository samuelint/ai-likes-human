import { invoke } from '@tauri-apps/api/tauri';

export async function isInferenceServerRunning(): Promise<boolean> {
  return invoke('is_server_up');
}

export async function getInferenceServerUrl(): Promise<string> {
  return invoke('get_inference_server_url');
}