import useSWR from 'swr';
import { fetchApiJson } from '@/lib/api-fetcher';
import { ExtensionInfoDto } from './extension.dto';



export function useAvailableExtensions() {
  const url = '/extension';
  const { data, error, isLoading } = useSWR<ExtensionInfoDto[]>(url, fetchApiJson);

  return {
    data,
    isLoading,
    error,
  };
}
