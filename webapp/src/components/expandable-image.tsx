import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogTrigger,
} from '@/components/ui/dialog';
import { cn } from '@/lib/utils';
import { ZoomIn } from 'lucide-react';
import { Button } from './ui/button';
import { Image, ImageProps } from './image';


export function ExpandableImage({ url, alt, className }: ImageProps) {
  return (
    <Dialog>
      <DialogTrigger className='relative'>
        <div className='absolute inset-0 flex items-center justify-center group-hover:opacity-100 opacity-0 group-hover:pointer-events-auto transition-opacity duration-200'>
          <Button variant='ghost'><ZoomIn/></Button>
        </div>
        <Image url={url} alt={alt} className='group-hover:opacity-50 group-hover:pointer-events-none transition-opacity duration-200'/>
      </DialogTrigger>
      <DialogContent className={cn('max-w-screen h-screen overflow-auto', className)}>
        <DialogHeader>
          <DialogTitle>{alt}</DialogTitle>
          <DialogDescription>
            <Image url={url} alt={alt} className='w-full' />
          </DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}