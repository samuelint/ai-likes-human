import useSWR from 'swr';
import { createApiJsonFetcher } from '../api-fetcher';
import { toThreadPreview } from './thread.type';


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

