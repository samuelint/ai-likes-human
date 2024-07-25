import { ImageAttachment as ImageDto } from '@/lib/assistant/image-attachment.type';
import { ExpandableImage } from './expandable-image';
import { Button } from './ui/button';
import { CircleXIcon } from 'lucide-react';


interface Props {
  image: ImageDto
  className?: string
  onRemoveClick?: (image: ImageDto) => void
}

export function ImageAttachment({ image, className, onRemoveClick }: Props) {
  return (
    <div className='relative rounded-xl overflow-hidden max-w-52 h-fit'>
      <ExpandableImage className={className} url={image.base64} alt={image.title} />
      { onRemoveClick && <Button
        variant='ghost'
        className='absolute top-1 left-1 p-0.5 rounded-full bg-white/90 h-fit w-fit'
        onClick={() => onRemoveClick(image)}
      >
        <CircleXIcon/>
      </Button>}
    </div>
  );
}
