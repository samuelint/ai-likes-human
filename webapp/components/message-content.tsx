import { Markdown } from './markdown';
import { ChatMessageContentDto } from './chat.type';


export interface ChatMessageProps {
  content: ChatMessageContentDto
}

export function MessageContent({ content }: ChatMessageProps) {

  return (
    <>
      { content.type === 'text' && <Markdown>{content.text.value}</Markdown> }
      { content.type === 'image_file' && JSON.stringify(content) }
      { content.type === 'image_url' && JSON.stringify(content) }
    </>
  );
}
