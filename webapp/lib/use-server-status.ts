import useSWR from 'swr';
import { apiJsonFetcher } from './api-fetcher';


interface ApiServerStatus {
  status: 'ok';
}

export function useServerStatus () {
  const { data, error, isLoading } = useSWR<ApiServerStatus>('/health', apiJsonFetcher);

  return {
    data,
    isLoading,
    error
  };
}