import useSWR from 'swr';
import { useCallback, useEffect, useState } from 'react';

import { useErrorNotification } from '@/app/_components/use-error-notification';
import { findConfiguration, upsertConfiguration } from '@/lib/api/tauri';



export function useConfigurationKV(key: string) {
  const { data, error: fetchError, isLoading, mutate: mutateCache } = useSWR(`/configuration/kv/${key}`, async () => await findConfiguration(key));

  const [error, setError] = useState<unknown | undefined>(undefined);

  useEffect(() => setError(fetchError), [fetchError]);

  const mutate = useCallback<(newValue: string) => Promise<void>>(async (newValue) => {
    try {
      const config = await upsertConfiguration({ key, value: newValue });
      mutateCache(config, { revalidate: false });
    } catch (e) {
      setError(e);
    }

  }, [key, mutateCache]);

  useErrorNotification(error);

  return {
    data,
    isLoading,
    error,
    mutate,
  };
}
