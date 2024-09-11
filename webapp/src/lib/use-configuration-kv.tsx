import useSWR from 'swr';
import { useCallback } from 'react';
import { findConfiguration, upsertConfiguration } from './tauri-command';



export function useConfigurationKV(key: string) {
  const { data, error, isLoading, mutate: mutateCache } = useSWR(`/configuration/kv/${key}`, async () => await findConfiguration(key));
  const mutate = useCallback<(newValue: string) => Promise<void>>(async (newValue) => {
    const config = await upsertConfiguration({ key, value: newValue });
    mutateCache(config, { revalidate: false });

  }, [key, mutateCache]);

  return {
    data,
    isLoading,
    error,
    mutate,
  };
}
