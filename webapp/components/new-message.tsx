'use client';

import { ChangeEvent, FormEventHandler, useRef } from 'react';
import SendButton from './ui/rich-text-input/send-button';
import RichTextInput from './ui/rich-text-input/rich-text-input';


export type OnSubmit = FormEventHandler<HTMLFormElement>;
export interface ChatNewMessageProps {
  placeholder?: string
  input?: string
  disabled?: boolean
  onChange?: (e: ChangeEvent<HTMLTextAreaElement>) => void
  onSubmit?: OnSubmit
}

export default function NewMessage({ placeholder = 'Type your message...', input, disabled, onChange, onSubmit }: ChatNewMessageProps) {
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
    <form ref={formRef} onSubmit={handleSubmit} className="bg-gray-100 dark:bg-gray-800 p-4 gap-2 flex items-center">
      <RichTextInput
        disabled={disabled}
        editable
        placeholder={placeholder}
        input={input}
        onChange={onChange}
        onKeyboardEvent={(event) => {
          if (event === 'Submit') programmaticSubmit();
        }}
      />
      <SendButton type="submit" />
    </form>
  );
}
