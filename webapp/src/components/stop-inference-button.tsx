import { forwardRef } from 'react';
import { Square } from 'lucide-react';
import { Button, ButtonProps } from '@/components/ui/button';
import { cn } from '@/lib/utils';


export const StopInferenceButton = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, ...props }, ref) => {
    return (
      <Button size="icon" className={cn('w-fit px-4 py-2 flex gap-4', className)} {...props} ref={ref}>
        <span>Stop</span>
        <Square className='w-6 h-6'/>
      </Button>
    );
  }
);
StopInferenceButton.displayName = 'StopInferenceButton';