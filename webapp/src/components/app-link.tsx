import { forwardRef, AnchorHTMLAttributes } from 'react';
import { ExternalLink } from 'lucide-react';
import { cn } from '@/lib/utils';

export const AppLink = forwardRef<HTMLAnchorElement, AnchorHTMLAttributes<HTMLAnchorElement>>(
  ({ className, target = '_blank', ...props }, ref) => {
    return (
      <a {...props} target={target} ref={ref} className={cn('text-blue-600 cursor-pointer inline-block', className)}>
        <div className='flex gap-1'>{props.children}{ target === '_blank' && <ExternalLink className='w-4 h-4'/>}</div>
      </a>
    );
  }
);
