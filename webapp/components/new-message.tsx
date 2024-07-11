'use client';

import { ChangeEvent, FormEventHandler, useRef } from 'react';
import SendButton from './ui/rich-text-input/send-button';
import RichTextInput from './ui/rich-text-input/rich-text-input';
import { StopInferenceButton } from './stop-inference-button';
import React from 'react';


export type OnSubmit = FormEventHandler<HTMLFormElement>;
export interface ChatNewMessageProps {
  placeholder?: string
  input?: string
  disabled?: boolean
  onChange?: (e: ChangeEvent<HTMLTextAreaElement>) => void
  onAbort?: () => void
  isLoading?: boolean
  onSubmit?: OnSubmit

  leftContent?: React.ReactNode
  rightContent?: React.ReactNode
  children?: React.ReactNode
}

export default function NewMessage({ placeholder = 'Type your message...', input, disabled, isLoading, onChange, onAbort, onSubmit, leftContent, rightContent, children }: ChatNewMessageProps) {
  const formRef = useRef<HTMLFormElement | null>(null);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
    onSubmit && onSubmit(event);
  };

  const programmaticSubmit = (): void => {
    if (formRef.current) {
      formRef.current.requestSubmit();
    }
  };

  const hasChildren = React.Children.count(children) > 0;
  const hastLeftContent = React.Children.count(leftContent) > 0;

  return (
    <form ref={formRef} onSubmit={handleSubmit} className="border rounded-xl p-2 flex flex-col items-center">
      <div className='flex items-center w-full'>
        { hastLeftContent &&
          <div className='pr-1'>
            { leftContent }
          </div> }
        <div className='relative w-full flex items-center'>
          <RichTextInput
            className='focus:outline-none p-0 relative'
            disabled={disabled}
            editable
            placeholder={placeholder}
            input={input}
            onChange={onChange}
            onKeyboardEvent={(event) => {
              if (event === 'Submit') programmaticSubmit();
            }}
          />
          <div className='flex flex-col gap-1 pl-2'>
            { isLoading ? <StopInferenceButton onClick={onAbort} disabled={disabled} /> : <SendButton disabled={disabled} type="submit" />}
            { rightContent }
          </div>
        </div>
      </div>
      {hasChildren && (
        <div className='w-full flex justify-start border-t border-gray-300 pt-2 gap-2'>
          {children}
        </div>
      )}
    </form>
  );
}
