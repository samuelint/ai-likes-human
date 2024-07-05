import useSWR from 'swr';
import { fetchApiJson } from './api-fetcher';


interface LLMModelFileDto {
  q4_gguf_filepath: string
  fp16_gguf_filepath: string
}

interface LLMModelIndex {
  name: string
  type: string
  local_files: LLMModelFileDto
}

export function useLocalModelsList() {
  const url = '/configuration/llm/local';
  const { data, error, isLoading } = useSWR<LLMModelIndex[]>(url, fetchApiJson);

  return {
    data,
    isLoading,
    error,
  };
}
