import useSWR from 'swr';
import { useOpenaiClient } from './openai-client';
import { keyBy } from 'lodash';


interface Props {
  threadId?: string
}

export function useThreadRuns({ threadId }: Props) {
  const openai = useOpenaiClient();
  const { data, error, isLoading, mutate } = useSWR(`/openai/v1/threads/${threadId}/runs`, async () => {
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

