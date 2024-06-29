import useSWR from 'swr';
import { apiFetcher } from './api-fetcher';


interface ConfigurationKv {
  key: string
  value: string
}

export function useConfigurationKV(key: string) {
  const { data, error, isLoading, mutate } = useSWR<ConfigurationKv>(`/configuration/kv/${key}`, apiFetcher);

  return {
    data,
    isLoading,
    error,
    mutate,
  };
}