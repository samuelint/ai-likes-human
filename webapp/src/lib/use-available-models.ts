export function useAvailableModels() {
  const data = ['openai:gpt-4o', 'openai:gpt-4o-mini', 'anthropic:claude-3-5-sonnet-20240620', 'local:llama3'] as readonly [string, ...string[]];
  return { data };
}