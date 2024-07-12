import { MessageContentPartParam, MessageCreateParams } from 'openai/resources/beta/threads/messages.mjs';
import { ImageAttachment } from '../image-attachment.type';


interface Props {
  input: string
  imageAttachments?: ImageAttachment[]
}

export function createUserMessage({ input, imageAttachments = [] }: Props): MessageCreateParams {
  const content: Array<MessageContentPartParam> = [{ type: 'text', text: input }];

  imageAttachments.forEach((imageAttachment) => {
    content.push({ type: 'image_url', image_url: { url: imageAttachment.base64, detail: 'low' } });
  });

  return { role: 'user', content };
}