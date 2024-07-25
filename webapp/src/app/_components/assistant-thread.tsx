
import { useErrorNotification } from '@/app/_components/use-error-notification';
import Chat from '@/components/chat';
import { useLLMModel } from '@/lib/use-llm-model';
import { useLlmTemperature } from '@/lib/use-llm-temperature';
import { useOpenAiAssistant } from '@/lib/assistant/use-openai-assistant';
import { useThreadRuns } from '@/lib/assistant/use-thread-runs';
import NewAssistantMessage from './new-assistant-message';
import { References } from '@/components/references';
import { useMessagesReferences } from '@/lib/use-messages-references';


interface Props {
  threadId: string
}

export default function AssistantThread({ threadId }: Props) {
  const { data: model } = useLLMModel();
  const { data: temperature } = useLlmTemperature();
  const { status, messages, error, input, submitMessage, handleInputChange, abort, addImageAttachments, imageAttachments, removeImageAttachment } = useOpenAiAssistant({ threadId, model, temperature });
  const { data: byIdRuns } = useThreadRuns({ threadId });
  const references = useMessagesReferences({ messages });

  useErrorNotification(error);

  const isLoading = status === 'in_progress';

  return (
    <Chat
      messages={messages}
      byIdRuns={byIdRuns}
      isLoading={isLoading}
    >
      <div className='flex justify-start'>
        <References references={references}/>
      </div>
      <NewAssistantMessage
        isLoading={isLoading}
        abort={abort}
        submitMessage={submitMessage}
        input={input}
        handleInputChange={handleInputChange}
        imageAttachments={imageAttachments}
        addImageAttachments={addImageAttachments}
        removeImageAttachment={removeImageAttachment}
      />
    </Chat>
  );
}
