import OpenAI from 'openai';

import { useCallback, useEffect, useRef, useState } from 'react';
import { useOpenaiClient } from './openai-client';
import { AssistantStream } from 'openai/lib/AssistantStream.mjs';
import { useImageAttachments } from './use-image-attachments';
import { createUserMessage } from './service/message-factory';


export type AssistantStatus = 'in_progress' | 'awaiting_message';
export type Message = OpenAI.Beta.Threads.Messages.Message;
export type MessageContent = OpenAI.Beta.Threads.Messages.MessageContent;
export type CreateMessage = OpenAI.Beta.Threads.Messages.MessageCreateParams;
type MessageDelta = OpenAI.Beta.Threads.Messages.MessageDelta;



interface Props {
    assistantId?: string;
    threadId: string;
    model?: string;
    temperature?: number;
}
export function useOpenAiAssistant({ assistantId = '', threadId, model, temperature }: Props) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const { imageAttachments, removeImageAttachment, addImageAttachments, setImageAttachments } = useImageAttachments();
  const [status, setStatus] = useState<AssistantStatus>('awaiting_message');
  const [error, setError] = useState<undefined | Error>(undefined);
  const streamRef = useRef<AssistantStream | null>(null);
  const abortControlerRef = useRef<AbortController | null>(null);
  const openai = useOpenaiClient();


  const setUnknownError = useCallback((e: unknown) => {
    if (e instanceof Error) setError(e);
    else setError(new Error(`${e}`));
  }, []);

  const handleInputChange = (
    event:
      | React.ChangeEvent<HTMLInputElement>
      | React.ChangeEvent<HTMLTextAreaElement>,
  ) => {
    setInput(event.target.value);
  };

  const streamRun = useCallback(async () => {
    if (status === 'in_progress') return;

    try {
      setStatus('in_progress');

      abortControlerRef.current = new AbortController();
      const signal = abortControlerRef.current.signal;

      await new Promise<void>((resolve, rejects) => {
        streamRef.current = openai.beta.threads.runs.stream(threadId, {
          model,
          assistant_id: assistantId,
          temperature,
        }, { signal })
          .on('messageCreated', (message: Message) => setMessages(messages => [...messages, message]))
          .on('messageDelta', (_delta: MessageDelta, snapshot: Message) => setMessages(messages => [
            ...messages.slice(0, messages.length - 1),
            snapshot
          ]))
          .on('messageDone', (message: Message) => [
            ...messages.slice(0, messages.length - 1),
            message
          ])
          .on('error', (error) => rejects(error))
          .on('abort', () => resolve())
          .on('end', () => resolve());
      });

    } catch (e) {
      setUnknownError(e);
      setMessages(messages => [
        ...messages.slice(0, messages.length - 1),
      ]);
    }
    finally {
      streamRef.current = null;
      setStatus('awaiting_message');
    }
  }, [assistantId, messages, model, openai.beta.threads.runs, setUnknownError, status, temperature, threadId]);

  const isLastMessageFromUser = useCallback(() => {
    return messages.at(-1)?.role === 'user';
  }, [messages]);

  useEffect(() => {
    if (isLastMessageFromUser()) {
      streamRun();
    }
  }, [streamRun, isLastMessageFromUser]);


  const append = useCallback(async (
    message?: CreateMessage,
  ) => {
    if (status === 'in_progress') throw new Error('Cannot append message while in progress');

    try {
      if (message) {
        const created_message = await openai.beta.threads.messages.create(
          threadId,
          message
        );
        setMessages(messages => [
          ...messages,
          created_message,
        ]);
      }

    } catch (e) {
      setUnknownError(e);
    }

  }, [openai.beta.threads.messages, threadId, setUnknownError, status]);

  const abort = useCallback(() => {
    if (abortControlerRef.current) {
      abortControlerRef.current.abort();
      abortControlerRef.current = null;
    }
  }, []);

  const streamRunRef = useRef(streamRun);

  useEffect(() => {
    streamRunRef.current = streamRun;
  }, [streamRun]);

  useEffect(() => {
    const fetchMessages = async () => {
      try {
        const newMessages = await openai.beta.threads.messages.list(threadId);
        setMessages(newMessages.data);
      } catch (e) {
        setUnknownError(e);
      }
    };
    fetchMessages();

  }, [openai.beta.threads.messages, threadId, setUnknownError]);

  const submitMessage = async (
    event?: React.FormEvent<HTMLFormElement>,
  ) => {
    event?.preventDefault?.();

    if (input === '') {
      return;
    }

    await append(createUserMessage({ input, imageAttachments }));
    setInput('');
    setImageAttachments([]);
  };

  return {
    input,
    setInput,
    messages,
    setMessages,
    threadId,
    error,
    status,
    submitMessage,
    handleInputChange,
    append,
    abort,
    imageAttachments,
    addImageAttachments,
    setImageAttachments,
    removeImageAttachment,
  };
}