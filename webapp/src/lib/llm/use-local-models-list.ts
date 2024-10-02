interface LLMModelIndex {
  name: string
  type: string
  local_path: string
}

export function useLocalModelsList(): LLMModelIndex[] {
  return [];
}
