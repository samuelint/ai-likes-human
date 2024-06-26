import { z } from 'zod';


export const ChatMessageSchema = z.object({
  id: z.string(),
  role: z.string(),
  createdAt: z.date().optional(),
  content: z.string(),
  tool_call_id: z.string().optional(),
});

export type ChatMessageDto = z.infer<typeof ChatMessageSchema>;

export function isAiMessage({ role }: Pick<ChatMessageDto, 'role'>): boolean {
  return role === 'ai' || role === 'assistant';
}