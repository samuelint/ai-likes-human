import useSWR from 'swr';
import { fetchApiJson } from './api-fetcher';


interface ApiServerStatus {
  status: 'ok';
}

interface Props {
  refreshInterval?: number;
}

export function useServerStatus ({ refreshInterval = 1000 }: Props = {}) {
  const { data, error, isLoading } = useSWR<ApiServerStatus>('/health', fetchApiJson, { refreshInterval });

  return {
    data,
    isLoading,
    error
  };
}