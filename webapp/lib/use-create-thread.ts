import { useCallback } from 'react';
import { useOpenaiClient } from './openai-client';
import { useListThreads } from './use-list-threads';
import { Thread } from 'openai/resources/beta/threads/threads.mjs';


export function useCreateThread(): () => Promise<Thread> {
  const openai = useOpenaiClient();
  const { revalidate } = useListThreads();

  return useCallback(async () => {
    const newThread = await openai.beta.threads.create();
    revalidate();

    return newThread;
  }, [openai.beta.threads, revalidate]);
}