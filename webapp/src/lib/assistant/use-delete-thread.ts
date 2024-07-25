import { useCallback } from 'react';
import { useOpenaiClient } from './openai-client';
import { useListThreads } from './use-list-threads';
import { useLocation } from 'wouter';


export function useDeleteThread(): (id: string) => Promise<void> {
  const openai = useOpenaiClient();
  const [_, setLocation] = useLocation();
  const { revalidate } = useListThreads();

  return useCallback(async (id) => {
    await openai.beta.threads.del(id);
    revalidate();
    setLocation('/');

  }, [openai.beta.threads, revalidate, setLocation]);
}