import appConfig from '@/app.config';

interface AvailableModelsDto {
  data: readonly [string, ...string[]];
  error?: Error
  isLoading?: boolean
}

export function useAvailableModels(): AvailableModelsDto {
  return {
    data: appConfig.available_llm_models,
    isLoading: false,
    error: undefined,
  };
}