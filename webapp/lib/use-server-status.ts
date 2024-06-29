import useSWR from 'swr';
import { apiFetcher } from './api-fetcher';


interface ApiServerStatus {
  status: 'ok';
}

export function useServerStatus () {
  const { data, error, isLoading } = useSWR<ApiServerStatus>('/health', apiFetcher);

  return {
    data,
    isLoading,
    error
  };
}