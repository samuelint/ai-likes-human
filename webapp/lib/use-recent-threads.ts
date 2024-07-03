import useSWR from 'swr';
import { apiJsonFetcher } from './api-fetcher';
import { toThreadPreview } from './thread.type';


interface Props {
  limit?: number
}

export function useRecentThreads({ limit = 2 }: Props = {}) {
  const { data, error, isLoading } = useSWR(`/assistant/openai/v1/threads?limit=${limit}`, apiJsonFetcher);

  return {
    data: data?.data.map(toThreadPreview), error, isLoading
  };
}

