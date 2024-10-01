import { useConfigurationKV } from '@/lib/configuration/use-configuration-kv';


export function useUsername() {
  const { data } = useConfigurationKV('USERNAME');
  return data?.value ?? '';
}