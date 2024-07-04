'use client';
import { useErrorNotification } from '@/app/_components/use-error-notification';

import Chat from '@/components/chat';
import { useOpenAiAssistant } from '@/lib/use-openai-assistant';


interface Props {
  threadId?: string
}

export default function AssistantThread({ threadId }: Props) {
  const { status, messages, error, input, submitMessage, handleInputChange } = useOpenAiAssistant({ threadId });
  useErrorNotification(error);

  const isLoading = status === 'in_progress';

  return (
    <Chat
      messages={messages}
      isLoading={isLoading}
      input={input}
      onChange={handleInputChange}
      onSubmit={submitMessage}
    />
  );
}
