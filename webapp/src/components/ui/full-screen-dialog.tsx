
import { DialogContent } from '@/components/ui/dialog';
import { cn } from '@/lib/utils';
import { forwardRef } from 'react';

export const FullScreenDialogContent = forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => {
  return (
    <DialogContent
      ref={ref}
      className={cn('max-w-[calc(100vw-2rem)] h-[calc(100vh-2rem)]', className)}
      {...props}
    />
  );
});

