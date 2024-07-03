import useSWR from 'swr';
import { apiJsonFetch, apiJsonFetcher } from './api-fetcher';
import { useCallback } from 'react';


interface ConfigurationKv {
  key: string
  value: string
}

export function useConfigurationKV(key: string) {
  const url = `/configuration/kv/${key}`;
  const { data, error, isLoading, mutate: mutateCache } = useSWR<ConfigurationKv>(url, apiJsonFetcher);

  const mutate = useCallback<(newValue: ConfigurationKv) => Promise<void>>(async (newValue) => {
    if (newValue.key !== key) throw new Error('Key mismatch');

    await apiJsonFetch(url, {
      method: 'PUT',
      body: JSON.stringify({ key, value: newValue.value }),
    });
    mutateCache({ ...data, ...newValue }, { revalidate: false });

  }, [key, url, mutateCache, data]);

  return {
    data,
    isLoading,
    error,
    mutate,
  };
}
