import { cn } from '@/lib/utils';


export interface ImageProps {
  url: string
  alt?: string
  className?: string
}

export function Image({ url, alt, className }: ImageProps) {
  return (
    <img className={cn('rounded-xl', className)} src={url} alt={alt} />
  );
}
