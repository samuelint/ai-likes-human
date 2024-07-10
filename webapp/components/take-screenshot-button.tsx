import { Button } from './ui/button';
import React from 'react';
import { ScreenshotIcon } from './icon/screenshot';
import { cn } from '@/lib/utils';


interface Props {
  className?: string
  onClick?: () => void
}

export function TakeScreenshotButton({ className, onClick }: Props) {
  return (
    <Button
      variant='ghost'
      onClick={onClick}
      className={cn('p-2', className)}
    >
      <ScreenshotIcon className='w-6 h-6'/>
    </Button>
  );
}
