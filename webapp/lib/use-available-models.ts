export function useAvailableModels() {
  const data = ['openai:gpt-4o', 'anthropic:claude-3-5-sonnet-20240620', 'local:phi3'] as readonly [string, ...string[]];
  return { data };
}