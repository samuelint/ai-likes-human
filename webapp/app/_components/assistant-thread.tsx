'use client';
import { useErrorNotification } from '@/app/_components/use-error-notification';
import Chat from '@/components/chat';
import { useCurrentModel } from '@/lib/use-current-model';
import { useLlmTemperature } from '@/lib/use-llm-temperature';
import { useOpenAiAssistant } from '@/lib/use-openai-assistant';
import { useThreadRuns } from '@/lib/use-thread-runs';
import { Tools } from './tools';
import NewMessage from '@/components/new-message';
import { ImageAttachment } from '@/components/image-attachment';


interface Props {
  threadId: string
}

export default function AssistantThread({ threadId }: Props) {
  const { data: model } = useCurrentModel();
  const { data: temperature } = useLlmTemperature();
  const { status, messages, error, input, submitMessage, handleInputChange, abort, addImageAttachments, imageAttachments } = useOpenAiAssistant({ threadId, model, temperature });
  const { data: byIdRuns } = useThreadRuns({ threadId });

  useErrorNotification(error);

  const isLoading = status === 'in_progress';

  return (
    <Chat
      messages={messages}
      byIdRuns={byIdRuns}
      isLoading={isLoading}
    >
      <NewMessage
        isLoading={isLoading}
        onAbort={abort}
        onSubmit={submitMessage}
        input={input}
        onChange={handleInputChange}
        leftContent={<Tools addImageAttachments={addImageAttachments} />}
        rightContent={
          <div className='flex w-full justify-end'>
            <span className='text-slate-400 text-xs'>{model}</span>
          </div>
        }
      >
        { imageAttachments.map((imageAttachment) => (
          <ImageAttachment key={imageAttachment.title} image={imageAttachment} />
        )) }
      </NewMessage>
    </Chat>
  );
}
