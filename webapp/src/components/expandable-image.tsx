import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogDescription,
  DialogTrigger,
} from '@/components/ui/dialog';
import { cn } from '@/lib/utils';
import { ZoomIn } from 'lucide-react';
import { Button } from './ui/button';
import { Image, ImageProps } from './image';

interface Props extends ImageProps {
  imageClassName?: string
}

export function ExpandableImage({ url, alt, className, imageClassName }: Props) {
  return (
    <Dialog>
      <DialogTrigger className='relative'>
        <div className='absolute inset-0 flex items-center justify-center group-hover:opacity-100 opacity-0 group-hover:pointer-events-auto transition-opacity duration-200'>
          <Button variant='ghost'><ZoomIn/></Button>
        </div>
        <Image url={url} alt={alt} className={cn('group-hover:opacity-50 group-hover:pointer-events-none transition-opacity duration-200', imageClassName)}/>
      </DialogTrigger>
      <DialogContent className={cn('max-w-screen h-screen overflow-auto', className)}>
        <DialogHeader>
        </DialogHeader>
        <DialogDescription>
          <Image url={url} alt={alt} className='w-full' />
        </DialogDescription>
      </DialogContent>
    </Dialog>
  );
}