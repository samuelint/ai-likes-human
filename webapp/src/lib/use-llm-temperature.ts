import { useConfigurationKV } from './use-configuration-kv';

export function useLlmTemperature() {
  const { data } = useConfigurationKV('DEFAULT_LLM_TEMPERATURE');

  const value = data?.value ? Number(data.value) : 0;

  return { data: value };
}