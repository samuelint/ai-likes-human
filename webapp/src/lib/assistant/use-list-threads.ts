import useSWR from 'swr';
import { createApiJsonFetcher } from '../api-fetcher';
import { toThreadPreview } from './thread.type';
import { openai_api_url } from '@/app.config';


interface Props {
  limit?: string
}

export function useListThreads({ limit = '10' }: Props = {}) {
  const { data, error, isLoading, mutate } = useSWR(`${openai_api_url}/threads`, createApiJsonFetcher({ queryParams: { limit } }));

  return {
    data: data?.data.map(toThreadPreview),
    error,
    isLoading,
    revalidate: () => mutate(),
  };
}

