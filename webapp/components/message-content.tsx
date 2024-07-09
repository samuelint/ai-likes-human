import { Markdown } from './markdown';
import { ChatMessageContentDto } from './chat.type';
import { Card } from './ui/card';
import { useMemo } from 'react';


export interface ChatMessageProps {
  content?: ChatMessageContentDto[]
}

export function MessageContent({ content }: ChatMessageProps) {
  const textContent = content?.filter((contentItem) => contentItem.type === 'text')?.map((contentItem) => contentItem.text.value).join('');
  const markdown = useMemo(() => <Markdown>{textContent}</Markdown>, [textContent]);
  const images = content?.filter((contentItem) => contentItem.type === 'image_file' || contentItem.type === 'image_url');

  return (
    <>
      { markdown }
      { images?.map((image, index) => <Card key={index}>{JSON.stringify(image)}</Card>)}
    </>
  );
}
