'use client';
import { Tools } from './tools';
import NewMessage from '@/components/new-message';
import { ImageAttachment as ImageDto } from '@/lib/image-attachment.type';
import { ImageAttachment } from '@/components/image-attachment';



interface Props {
  input: string
  submitMessage: (event?: React.FormEvent<HTMLFormElement>) => void
  isLoading?: boolean
  model?: string
  imageAttachments?: ImageDto[]
  addImageAttachments: (imageAttachment: ImageDto[]) => void
  removeImageAttachment: (imageAttachment: ImageDto) => void
  abort?: () => void
  handleInputChange: (event: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => void
}

export default function NewAssistantMessage({ input, submitMessage, handleInputChange, abort, model, isLoading, imageAttachments = [], addImageAttachments, removeImageAttachment }: Props) {
  return (
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
        <ImageAttachment key={imageAttachment.title} image={imageAttachment} onRemoveClick={removeImageAttachment} />
      )) }
    </NewMessage>
  );
}
