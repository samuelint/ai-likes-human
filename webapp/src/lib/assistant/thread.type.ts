import { Thread } from 'openai/resources/beta/threads/threads.mjs';
import { toDate } from '@/lib/utils/date';


export interface ThreadPreviewDto {
  id: string
  created_at: Date
  title: string
  assistantId?: string
}

export function toThreadPreview(thread: Thread): ThreadPreviewDto {
  return {
    id: thread.id,
    created_at: toDate(thread.created_at),
    // @ts-expect-error - Metadata (from lib) is not typed
    title: thread.metadata?.['title'] ?? '',
    // @ts-expect-error - Metadata (from lib) is not typed
    assistantId: thread.metadata?.assistantId ?? undefined,
  };
}
