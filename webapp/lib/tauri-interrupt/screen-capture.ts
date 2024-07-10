import { invoke } from '@tauri-apps/api/tauri';


export async function captureBase64Screens(): Promise<string[]> {
  const base64Images = await invoke<string[]>('capture_screen');

  return base64Images;
}