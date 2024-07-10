import { invoke } from '@tauri-apps/api/tauri';


export async function captureBase64Screens(): Promise<string[]> {
  const base64Images = await invoke<string[]>('capture_screen', { maxSize: 1024 });

  return base64Images;
}