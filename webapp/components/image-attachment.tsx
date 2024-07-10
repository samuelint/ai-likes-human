import { ImageAttachment as Image } from '@/lib/image-attachment.type';
import { Base64Image } from './base64-image';
import { Card } from './ui/card';


interface Props {
  image: Image
  className?: string
}

export function ImageAttachment({ image, className }: Props) {
  return (
    <Card>
      <Base64Image className={className} base64={image.base64} alt={image.title} />
    </Card>
  );
}
