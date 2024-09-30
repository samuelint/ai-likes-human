interface AvailableModelsDto {
  data: readonly [string, ...string[]];
  error?: Error
  isLoading?: boolean
}

const DEFAULT_MODELS: readonly [string, ...string[]] = ['openai:gpt-4o', 'openai:gpt-4o-mini', 'local:llama3'];

export function useAvailableModels(): AvailableModelsDto {
  return {
    data: DEFAULT_MODELS,
    isLoading: false,
    error: undefined,
  };
}