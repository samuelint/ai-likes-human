'use client';
import { useErrorNotification } from '@/app/_components/use-error-notification';
import Chat from '@/components/chat';
import { Button } from '@/components/ui/button';
import { useAssistant } from '@ai-sdk/react';
import { useCallback } from 'react';
import { useLocalServerLogForward } from '@/lib/use-local-server-log-forward';


export default function Assistant() {
  useLocalServerLogForward();

  const { status, messages, setMessages, error, input, submitMessage, handleInputChange } = useAssistant({
    api: '/api/assistant',
  });
  useErrorNotification(error);

  const isLoading = status === 'in_progress';

  const clear = useCallback(() => {
    setMessages([ ]);
  }, [setMessages]);

  return (
    <Chat
      messages={messages}
      isLoading={isLoading}
      input={input}
      onChange={handleInputChange}
      onSubmit={submitMessage}
    >
      <div className="flex gap-4">
        <Button size='sm' onClick={() => clear()}>Clear</Button>
      </div>
    </Chat>
  );
}
