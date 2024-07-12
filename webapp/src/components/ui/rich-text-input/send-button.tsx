import { Button, ButtonProps } from '@/components/ui/button';
import { cn } from '@/lib/utils';

import { ForwardRefRenderFunction, forwardRef } from 'react';



export interface SendButtonProps extends ButtonProps { }

const SendButtonInner: ForwardRefRenderFunction<HTMLButtonElement, SendButtonProps> = ({ className, ...props }: SendButtonProps, ref) => (
  <Button size="icon" className={cn('w-fit px-4 py-2 flex gap-4', className)} {...props} ref={ref}>
    <span>Send</span>
    <span className="bg-slate-200/25 rounded px-1">⌘ ↵</span>
  </Button>
);

export const SendButton = forwardRef<HTMLButtonElement, SendButtonProps>(SendButtonInner);