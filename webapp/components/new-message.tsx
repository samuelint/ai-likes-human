'use client';

import { ChangeEvent, FormEventHandler, useRef } from 'react';
import SendButton from './ui/rich-text-input/send-button';
import RichTextInput from './ui/rich-text-input/rich-text-input';
import { StopInferenceButton } from './stop-inference-button';


export type OnSubmit = FormEventHandler<HTMLFormElement>;
export interface ChatNewMessageProps {
  placeholder?: string
  input?: string
  disabled?: boolean
  onChange?: (e: ChangeEvent<HTMLTextAreaElement>) => void
  onAbort?: () => void
  isLoading?: boolean
  onSubmit?: OnSubmit
  children?: React.ReactNode
}

export default function NewMessage({ placeholder = 'Type your message...', input, disabled, isLoading, onChange, onAbort, onSubmit, children }: ChatNewMessageProps) {
  const formRef = useRef<HTMLFormElement | null>(null);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
    onSubmit && onSubmit(event);
  };

  const programmaticSubmit = (): void => {
    if (formRef.current) {
      formRef.current.requestSubmit();
    }
  };

  return (
    <form ref={formRef} onSubmit={handleSubmit} className="relative border rounded-xl p-4 gap-2 flex items-center">
      <RichTextInput
        className='focus:outline-none'
        disabled={disabled}
        editable
        placeholder={placeholder}
        input={input}
        onChange={onChange}
        onKeyboardEvent={(event) => {
          if (event === 'Submit') programmaticSubmit();
        }}
      />
      <div className='flex flex-col gap-1'>
        { isLoading ? <StopInferenceButton onClick={onAbort} /> : <SendButton type="submit" />}
        {/* <StopInferenceButton onClick={onAbort} />
        <SendButton type="submit" /> */}
        { children }
      </div>
    </form>
  );
}
