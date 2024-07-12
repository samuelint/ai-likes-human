import { ImageAttachment as ImageDto } from '@/lib/image-attachment.type';
import { ExpandableImage } from './image';
import { Button } from './ui/button';
import { CircleXIcon } from 'lucide-react';


interface Props {
  image: ImageDto
  className?: string
  onRemoveClick?: (image: ImageDto) => void
}

export function ImageAttachment({ image, className, onRemoveClick }: Props) {
  return (
    <div className='relative rounded-xl border border-slate-600 overflow-hidden'>
      <ExpandableImage className={className} url={image.base64} alt={image.title} />
      { onRemoveClick && <Button
        variant='ghost'
        className='absolute top-1 left-1 p-1 rounded-full bg-white/50 h-fit w-fit'
        onClick={() => onRemoveClick(image)}
      >
        <CircleXIcon/>
      </Button>}

    </div>
  );
}
