import { LLM_VENDORS_CONFIGURATIONS } from '@/app.config';
import { findConfiguration } from '../core-api/tauri';

export async function isAtLeastOneLlmApiKeySet(): Promise<boolean> {
  for (const vendor of LLM_VENDORS_CONFIGURATIONS) {
    if (!vendor.api_key_key) {
      continue;
    }
    const config = await findConfiguration(vendor.api_key_key);
    if (config?.value) {
      return true;
    }
  }

  return false;
}