import OpenAI from 'openai';

import { useCallback, useEffect, useRef, useState } from 'react';
import { useOpenaiClient } from './openai-client';
import { ImageAttachment } from '@/lib/image-attachment.type';
import { AssistantStream } from 'openai/lib/AssistantStream.mjs';


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
    initialInput?: string;
}
export function useOpenAiAssistant({ assistantId = '', threadId, model = 'openai:gpt-3.5-turbo', temperature, initialInput }: Props) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState(initialInput ?? '');
  const [imageAttachments, setImageAttachments] = useState<ImageAttachment[]>([]);
  const [status, setStatus] = useState<AssistantStatus>('awaiting_message');
  const [error, setError] = useState<undefined | Error>(undefined);
  const streamRef = useRef<AssistantStream | null>(null);
  const abortControlerRef = useRef<AbortController | null>(null);
  const openai = useOpenaiClient();

  const setUnknownError = useCallback((e: unknown) => {
    if (e instanceof Error) setError(e);
    else setError(new Error(`${e}`));
  }, []);

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

  const handleInputChange = (
    event:
      | React.ChangeEvent<HTMLInputElement>
      | React.ChangeEvent<HTMLTextAreaElement>,
  ) => {
    setInput(event.target.value);
  };

  const streamRun = useCallback(async () => {
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
          .on('messageDelta', (_delta: MessageDelta, snapshot: Message) => setMessages(messages => {
            return [
              ...messages.slice(0, messages.length - 1),
              snapshot
            ];
          }))
          .on('messageDone', (message: Message) => {
            return [
              ...messages.slice(0, messages.length - 1),
              message
            ];
          })
          .on('error', (error) => rejects(error))
          .on('abort', () => resolve())
          .on('end', () => resolve());
      });

    } catch (e) {
      setUnknownError(e);
      setMessages(messages => {
        return [
          ...messages.slice(0, messages.length - 1),
        ];
      });
    }
    finally {
      streamRef.current = null;
      setStatus('awaiting_message');
    }
  }, [assistantId, messages, model, openai.beta.threads.runs, setUnknownError, temperature, threadId]);

  useEffect(() => {
    if (messages.at(-1)?.role === 'user') {
      streamRun();
    }
  }, [messages, streamRun]);


  const append = useCallback(async (
    message?: CreateMessage,
  ) => {
    setInput('');

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

  }, [openai.beta.threads.messages, threadId, setUnknownError]);

  const abort = useCallback(() => {
    if (abortControlerRef.current) {
      abortControlerRef.current.abort();
      abortControlerRef.current = null;
    }
  }, []);


  const submitMessage = async (
    event?: React.FormEvent<HTMLFormElement>,
  ) => {
    event?.preventDefault?.();

    if (input === '') {
      return;
    }

    const content = [input];

    imageAttachments.forEach((imageAttachment) => {
      content.push(`![${imageAttachment.title}](${imageAttachment.base64})`);
    });

    append({ role: 'user', content: content.join('\n') });
  };

  const addImageAttachments = (newImageAttachments: ImageAttachment[]) => {
    setImageAttachments(imageAttachments => [...imageAttachments, ...newImageAttachments]);
  };

  return { input, setInput, messages, setMessages, threadId, error, status, submitMessage, handleInputChange, append, abort, imageAttachments, addImageAttachments, setImageAttachments };
}