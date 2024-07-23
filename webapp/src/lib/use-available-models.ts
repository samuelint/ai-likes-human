import useSWR from 'swr';
import { fetchApiJson } from './api-fetcher';


interface AvailableModelsDto {
  data: readonly [string, ...string[]];
  error?: Error
  isLoading?: boolean
}

const DEFAULT_MODELS: readonly [string, ...string[]] = ['local:llama3'];

export function useAvailableModels(): AvailableModelsDto {
  const url = '/configuration/available-models';
  const { data, error, isLoading } = useSWR<readonly [string, ...string[]]>(url, fetchApiJson);

  return {
    data: data ?? DEFAULT_MODELS,
    isLoading,
    error,
  };
}