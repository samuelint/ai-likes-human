import { Tools } from './tools';
import NewMessage from '@/components/new-message';
import { ImageAttachment as ImageDto } from '@/lib/assistant/image-attachment.type';
import { ImageAttachment } from '@/components/image-attachment';
import AssistantModelSettingsModal from './assistant-model-settings-modal';
import { useClipboardPasteImage } from '@/lib/assistant/use-clipboard-paste-image';


interface Props {
  input: string
  submitMessage: (event?: React.FormEvent<HTMLFormElement>) => void
  isLoading?: boolean
  imageAttachments?: ImageDto[]
  addImageAttachments: (imageAttachment: ImageDto[]) => void
  removeImageAttachment: (imageAttachment: ImageDto) => void
  abort?: () => void
  handleInputChange: (event: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => void
}

export default function NewAssistantMessage({ input, submitMessage, handleInputChange, abort, isLoading, imageAttachments = [], addImageAttachments, removeImageAttachment }: Props) {
  useClipboardPasteImage((image) => addImageAttachments([image]));

  return (
    <div className='w-full flex flex-col gap-0.5'>
      <NewMessage
        isLoading={isLoading}
        onAbort={abort}
        onSubmit={submitMessage}
        input={input}
        onChange={handleInputChange}
        leftContent={<Tools addImageAttachments={addImageAttachments} />}
      >
        { imageAttachments.map((imageAttachment) => (
          <ImageAttachment key={imageAttachment.title} image={imageAttachment} onRemoveClick={removeImageAttachment} />
        )) }
      </NewMessage>
      <div className='flex w-full justify-end'>
        <AssistantModelSettingsModal />
      </div>
    </div>

  );
}
