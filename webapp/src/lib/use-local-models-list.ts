// import useSWR from 'swr';
// import { fetchApiJson } from './api-fetcher';


interface LLMModelIndex {
  name: string
  type: string
  local_path: string
}

export function useLocalModelsList(): LLMModelIndex[] {
  // const url = '/configuration/llm/local';
  // const { data, error, isLoading } = useSWR<LLMModelIndex[]>(url, fetchApiJson);

  return [];
}
