import { Thread } from 'openai/resources/beta/threads/threads.mjs';


export interface ThreadPreviewDto {
  id: string
  created_at: Date
  title: string
}

export function toThreadPreview(thread: Thread): ThreadPreviewDto {
  return {
    id: thread.id,
    created_at: new Date(thread.created_at * 1000),
    // @ts-expect-error - Metadata (from lib) is not typed
    title: thread.metadata?.['title'] ?? ''
  };
}
