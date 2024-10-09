import { invoke } from './invoke';

export interface ConfigurationItemDto {
  key: string
  value: string
  hint?: string
}

export async function findConfiguration(key: string): Promise<ConfigurationItemDto | undefined> {
  return await invoke<ConfigurationItemDto | undefined>('find_configuration', { key });
}

interface UpdateConfigurationArgs {
  key: string
  value: string
}

export async function upsertConfiguration({ key, value }: UpdateConfigurationArgs): Promise<ConfigurationItemDto> {
  return await invoke<ConfigurationItemDto>('upsert_configuration', { key, value });
}
