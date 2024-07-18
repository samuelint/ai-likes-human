import OpenAI from 'openai';
import { MessageReference } from './message-reference.type';

interface Props {
  messages: Pick<OpenAI.Beta.Threads.Messages.Message, 'role' | 'content'>[]
}

const markdown_links_pattern = /\[(.*?)\]\((.*?)\)/g;

export function useMessagesReferences({ messages }: Props): MessageReference[] {

  return messages
    .filter((message) => message.role === 'assistant')
    .reduce<MessageReference[]>((acc, message) => {
      const all_content = getMessageStringContent(message.content);

      return [...acc, ...toMessageReferences(all_content)];
    }, []);

}

function toMessageReferences(content: string): MessageReference[] {
  const links = [...content.matchAll(markdown_links_pattern)];

  return links.map(toMessageReference);
}

function toMessageReference([_, title, link]: string[]): MessageReference {

  return {
    title,
    link,
  };
}

function getMessageStringContent(content: OpenAI.Beta.Threads.Messages.MessageContent[]): string {
  return content.reduce((acc, content) => {
    if (content.type === 'text') {
      return acc + content.text.value;
    }
    return acc;
  }, '');
}