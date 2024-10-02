import { LLM_API_KEYS_KEYS } from '@/app.config';
import { findConfiguration } from '../core-api/tauri';

export async function isAtLeastOneLlmApiKeySet(): Promise<boolean> {
  for (const key of LLM_API_KEYS_KEYS) {
    const config = await findConfiguration(key);
    if (config?.value) {
      return true;
    }
  }

  return false;
}