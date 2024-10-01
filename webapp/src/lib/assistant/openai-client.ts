import { openai_api_url } from '@/app.config';
import OpenAI from 'openai';
import { useMemo } from 'react';


const default_openai = new OpenAI({
  baseURL: openai_api_url,
  apiKey: 'some',
  dangerouslyAllowBrowser: true,
});

interface Props {
  openai: OpenAI
}

export function useOpenaiClient({ openai }: Props = { openai: default_openai }) {
  return useMemo(() => openai, [openai]);
}