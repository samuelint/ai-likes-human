import { useConfigurationKV } from './use-configuration-kv';


export function useCurrentModel() {
  const { data } = useConfigurationKV('DEFAULT_LLM_MODEL');

  return { data: data?.value };
}