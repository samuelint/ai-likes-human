import { Markdown } from './markdown';
import { ChatMessageContentDto } from './chat.type';
import { Card } from './ui/card';


export interface ChatMessageProps {
  content?: ChatMessageContentDto[]
}

export function MessageContent({ content }: ChatMessageProps) {
  const textContent = content?.filter((contentItem) => contentItem.type === 'text')?.map((contentItem) => contentItem.text.value).join('');
  const images = content?.filter((contentItem) => contentItem.type === 'image_file' || contentItem.type === 'image_url');
  return (
    <>
      <Markdown>{textContent}</Markdown>
      { images?.map((image, index) => <Card key={index}>{JSON.stringify(image)}</Card>)}
    </>
  );
}
