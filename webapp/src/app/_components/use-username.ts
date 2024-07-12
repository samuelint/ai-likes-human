import { useConfigurationKV } from '@/lib/use-configuration-kv';


export function useUsername() {
  const { data } = useConfigurationKV('USERNAME');
  return data?.value ?? '';
}