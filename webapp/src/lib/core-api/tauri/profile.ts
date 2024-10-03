import { invoke } from '@tauri-apps/api/tauri';

interface ProfileDto {
  name: string;
  prompt: string;
}

export async function getSelectedProfiles(): Promise<ProfileDto[]> {
  return await invoke<ProfileDto[]>('selected_profiles');
}
