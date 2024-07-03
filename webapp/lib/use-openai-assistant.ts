import OpenAI from 'openai';

import { useState } from 'react';
import { useOpenaiClient } from './openai-client';


export type AssistantStatus = 'in_progress' | 'awaiting_message';
export type Message = OpenAI.Beta.Threads.Messages.Message;
export type MessageContent = OpenAI.Beta.Threads.Messages.MessageContent;
export type CreateMessage = OpenAI.Beta.Threads.Messages.MessageCreateParams;
type MessageDelta = OpenAI.Beta.Threads.Messages.MessageDelta;


interface Props {
    assistantId?: string;
}
export function useOpenAiAssistant({ assistantId = '' }: Props = {}) {
  const [messages, setMessages] = useState<Message[]>([ ]);
  const [input, setInput] = useState('');
  const [threadId, setThreadId] = useState<string | undefined>(undefined);
  const [status, setStatus] = useState<AssistantStatus>('awaiting_message');
  const [error, setError] = useState<undefined | Error>(undefined);
  const openai = useOpenaiClient();

  const handleInputChange = (
    event:
      | React.ChangeEvent<HTMLInputElement>
      | React.ChangeEvent<HTMLTextAreaElement>,
  ) => {
    setInput(event.target.value);
  };

  const append = async (
    message: CreateMessage,
  ) => {
    try {
      setStatus('in_progress');

      let local_threadId = threadId;
      if (!local_threadId) {
        const thread = await openai.beta.threads.create();
        local_threadId = thread.id;
        setThreadId(local_threadId);
      }

      const created_message = await openai.beta.threads.messages.create(
        local_threadId,
        message
      );
      setMessages(messages => [
        ...messages,
        created_message,
      ]);

      await new Promise<void>((resolve, rejects) => openai.beta.threads.runs.stream(local_threadId, {
        model: 'openai:gpt-3.5-turbo',
        assistant_id: assistantId,
      })
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
        .on('end', () => resolve())
      );

    } catch (e) {
      if (e instanceof Error) setError(e);
      else setError(new Error(`${e}`));
    }
    finally {
      setStatus('awaiting_message');
    }
  };

  const submitMessage = async (
    event?: React.FormEvent<HTMLFormElement>,
  ) => {
    event?.preventDefault?.();

    if (input === '') {
      return;
    }

    append({ role: 'user', content: input });
  };

  return { input, setInput, messages, setMessages, threadId, error, status, submitMessage, handleInputChange, append };
}