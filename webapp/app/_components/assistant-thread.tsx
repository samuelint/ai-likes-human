'use client';
import { useErrorNotification } from '@/app/_components/use-error-notification';
import Chat from '@/components/chat';
import { useCurrentModel } from '@/lib/use-current-model';
import { useLlmTemperature } from '@/lib/use-llm-temperature';
import { useOpenAiAssistant } from '@/lib/use-openai-assistant';


interface Props {
  threadId?: string
}

export default function AssistantThread({ threadId }: Props) {
  const { data: model } = useCurrentModel();
  const { data: temperature } = useLlmTemperature();
  const { status, messages, error, input, submitMessage, handleInputChange } = useOpenAiAssistant({ threadId, model, temperature });
  useErrorNotification(error);

  const isLoading = status === 'in_progress';

  return (
    <Chat
      messages={messages}
      isLoading={isLoading}
      input={input}
      onChange={handleInputChange}
      onSubmit={submitMessage}
      details={<span className='text-slate-400 text-xs'>{model}</span>}
    />
  );
}
