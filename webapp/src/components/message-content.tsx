import { Markdown } from './markdown';
import { MessageContent as MessageContentDto } from '@/lib/assistant/use-openai-assistant';
import { useMemo } from 'react';
import { ExpandableImage } from './expandable-image';


export interface ChatMessageProps {
  content?: MessageContentDto[]
}

export function MessageContent({ content }: ChatMessageProps) {
  const textContent = content?.filter((contentItem) => contentItem.type === 'text')?.map((contentItem) => contentItem.text.value).join('');
  const markdown = useMemo(() => <Markdown>{textContent}</Markdown>, [textContent]);
  const images = content?.filter((contentItem) => contentItem.type === 'image_url');
  const hasAttachments = images?.length && images.length > 0;

  return (
    <>
      { markdown }
      { hasAttachments ? <div className='border-t border-gray-300 py-4'>
        { images.map((image, index) => <ExpandableImage key={index} url={image.image_url.url} alt={`${index}`} imageClassName='max-w-64' />) }
      </div> : null}
    </>
  );
}
