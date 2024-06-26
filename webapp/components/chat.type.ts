// import { z } from 'zod';

import { Message, MessageContent } from '@/app/_components/use-openai-assistant';


// export const ChatMessageSchema = z.object({
//   id: z.string(),
//   role: z.string(),
//   createdAt: z.number().optional(),
//   content: z.any(),
// });

// export type ChatMessageDto = z.infer<typeof ChatMessageSchema>;
export type ChatMessageDto = Message;
export type ChatMessageContentDto = MessageContent;

export function isAiMessage({ role }: Pick<ChatMessageDto, 'role'>): boolean {
  return role === 'assistant';
}