import useSWR from 'swr';
import { apiJsonFetcher } from './api-fetcher';


interface ApiServerStatus {
  status: 'ok';
}

interface Props {
  refreshInterval?: number;
}

export function useServerStatus ({ refreshInterval = 1000 }: Props = {}) {
  const { data, error, isLoading } = useSWR<ApiServerStatus>('/health', apiJsonFetcher, { refreshInterval });

  return {
    data,
    isLoading,
    error
  };
}