import useSWR from 'swr';

import { toThreadPreview } from './thread.type';
import { createApiJsonFetcher } from '@/lib/api/api-fetcher';


interface Props {
  limit?: string
}

export function useListThreads({ limit = '10' }: Props = {}) {
  const { data, error, isLoading, mutate } = useSWR('/openai/v1/threads', createApiJsonFetcher({ queryParams: { limit } }));

  return {
    data: data?.data.map(toThreadPreview),
    error,
    isLoading,
    revalidate: () => mutate(),
  };
}

