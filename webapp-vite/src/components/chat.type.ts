import { Message, MessageContent } from '@/lib/use-openai-assistant';


export type ChatMessageDto = Message;
export type ChatMessageContentDto = MessageContent;

export function isAiMessage({ role }: Pick<ChatMessageDto, 'role'>): boolean {
  return role === 'assistant';
}