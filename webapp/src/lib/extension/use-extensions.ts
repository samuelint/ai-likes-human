import useSWR from 'swr';
import { fetchApiJson, fetchApi } from '@/lib/api-fetcher';
import { ExtensionStateDto } from './extension.dto';
import { useCallback, useEffect, useState } from 'react';
import { OnExtensionSubmit } from '@/components/add-local-extension-form';



export function useExtensions() {
  const { data, error: fetchAvailableError, isLoading, mutate } = useSWR<ExtensionStateDto[]>('/extension', fetchApiJson);
  const [error, setError] = useState<Error | undefined>(undefined);

  useEffect(() => {
    setError(fetchAvailableError);
  }, [fetchAvailableError]);

  const upload = useCallback<OnExtensionSubmit>(async (data) => {
    const file = new File([data.file], data.file.name, { type: 'application/zip' });
    const formData = new FormData();
    formData.append('file', file);

    try {
      const result = await fetchApi('/extension/upload', {
        method: 'POST',
        body: formData,
      });

      const newExtensionInfo = await result.json() as ExtensionStateDto;
      mutate((prevData) => (prevData || []).concat(newExtensionInfo), true);
    } catch (error) {
      setError(error as Error);
    }

  }, [mutate]);

  const remove = useCallback(async (extension: Pick<ExtensionStateDto, 'name'>) => {
    try {
      await fetchApi(`/extension/${extension.name}`, {
        method: 'DELETE',
      });
      mutate((prevData) => (prevData || []).filter((ex) => ex.name !== extension.name));
    } catch (error) {
      setError(error as Error);
    }

  }, [mutate]);

  const load = useCallback(async (extension: Pick<ExtensionStateDto, 'name'>) => {
    try {
      const result = await fetchApiJson<ExtensionStateDto>(`/extension/${extension.name}/load`, {
        method: 'POST',
      });

      mutate((prevData) => (prevData || []).map((ex) =>
        ex.name === result.name ? result : ex
      ));
    } catch (error) {
      setError(error as Error);
    }
  }, [mutate]);

  const unload = useCallback(async (extension: Pick<ExtensionStateDto, 'name'>) => {
    try {
      const result = await fetchApiJson<ExtensionStateDto>(`/extension/${extension.name}/unload`, {
        method: 'POST',
      });

      mutate((prevData) => (prevData || []).map((ex) =>
        ex.name === result.name ? result : ex
      ));
    } catch (error) {
      setError(error as Error);
    }
  }, [mutate]);

  return {
    data,
    isLoading,
    error,
    upload,
    load,
    unload,
    remove,
  };
}
