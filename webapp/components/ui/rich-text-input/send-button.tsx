import { Button, ButtonProps } from '@/components/ui/button';

import { ForwardRefRenderFunction, forwardRef } from 'react';
import { twMerge } from 'tailwind-merge';


export interface SendButtonProps extends ButtonProps { }

const SendButton: ForwardRefRenderFunction<HTMLButtonElement, SendButtonProps> = ({ className, ...props }: SendButtonProps, ref) => (
  <Button size="icon" className={twMerge('w-fit px-4 py-2 flex gap-4', className)} {...props} ref={ref}>
    <span>Send</span>
    <span className="bg-slate-200/25 rounded px-1">⌘ ↵</span>
  </Button>
);

export default forwardRef<HTMLButtonElement, SendButtonProps>(SendButton);