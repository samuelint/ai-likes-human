import OpenAI from 'openai';


const default_openai = new OpenAI({
  baseURL: 'http://localhost:8000/assistant/openai/v1',
  apiKey: 'some',
  dangerouslyAllowBrowser: true,
});

interface Props {
  openai: OpenAI
}

export function useOpenaiClient({ openai }: Props = { openai: default_openai }) {
  return openai;
}