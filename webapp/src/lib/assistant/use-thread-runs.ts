import useSWR from 'swr';
import { useOpenaiClient } from './openai-client';
import { keyBy } from 'lodash';
import { openai_api_url } from '@/app.config';


interface Props {
  threadId?: string
}

export function useThreadRuns({ threadId }: Props) {
  const openai = useOpenaiClient();
  const { data, error, isLoading, mutate } = useSWR(`${openai_api_url}/threads/${threadId}/runs`, async () => {
    if (!threadId) return {};
    const runsPage = await openai.beta.threads.runs.list(threadId);
    return keyBy(runsPage.data, 'id');
  });

  return {
    data: data,
    error,
    isLoading,
    revalidate: () => mutate(),
  };
}

