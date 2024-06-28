'use client';
import OpenAI from 'openai';
import { useErrorNotification } from '@/app/_components/use-error-notification';

import Chat from '@/components/chat';
import { Button } from '@/components/ui/button';
import { useCallback } from 'react';
import { useOpenAiAssistant } from './use-openai-assistant';


export default function Assistant() {
  const { status, messages, setMessages, error, input, submitMessage, handleInputChange } = useOpenAiAssistant({
    openai: new OpenAI({
      baseURL: 'http://localhost:8000/assistant/openai/v1',
      apiKey: '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',
      dangerouslyAllowBrowser: true,
    })
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
