'use client';
import { useErrorNotification } from '@/app/_components/use-error-notification';

import Chat from '@/components/chat';
import { Button } from '@/components/ui/button';
import { useCallback } from 'react';
import { useOpenAiAssistant } from '@/lib/use-openai-assistant';



export default function Assistant() {
  const { status, messages, setMessages, error, input, submitMessage, handleInputChange } = useOpenAiAssistant();
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
