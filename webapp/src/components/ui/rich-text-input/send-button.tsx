import { Button, ButtonProps } from '@/components/ui/button';
import { cn } from '@/lib/utils';

import { forwardRef, useMemo } from 'react';
import { useSendKeyboardShortcut } from './use-send-keyboard-shortcut';

export interface SendButtonProps extends ButtonProps { }

export const SendButton = forwardRef<HTMLButtonElement, SendButtonProps>(({ className, ...props }: SendButtonProps, ref) => {
  const sendKeyboardShortcut = useSendKeyboardShortcut();
  const strSendText = useMemo(() => sendKeyboardShortcut.join(' '), [sendKeyboardShortcut]);

  return (
    <Button size="icon" className={cn('w-fit px-4 py-2 flex gap-4', className)} {...props} ref={ref}>
      <span>Send</span>
      <span className="bg-slate-200/25 rounded px-1">{strSendText}</span>
    </Button>
  );
});