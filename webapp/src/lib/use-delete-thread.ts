import { useCallback } from 'react';
import { useOpenaiClient } from './openai-client';
import { useListThreads } from './use-list-threads';


export function useDeleteThread(): (id: string) => Promise<void> {
  const openai = useOpenaiClient();
  const { revalidate } = useListThreads();

  return useCallback(async (id) => {
    await openai.beta.threads.del(id);
    revalidate();

  }, [openai.beta.threads, revalidate]);
}