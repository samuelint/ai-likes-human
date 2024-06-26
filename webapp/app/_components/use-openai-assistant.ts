import OpenAI from 'openai';

import { useRef, useState } from 'react';


export type AssistantStatus = 'in_progress' | 'awaiting_message';
export type Message = OpenAI.Beta.Threads.Messages.Message;
export type MessageContent = OpenAI.Beta.Threads.Messages.MessageContent;
export type CreateMessage = OpenAI.Beta.Threads.Messages.MessageCreateParams;
type Text = OpenAI.Beta.Threads.Messages.Text;
type MessageDelta = OpenAI.Beta.Threads.Messages.MessageDelta;
type TextDelta = OpenAI.Beta.Threads.Messages.TextDelta;


interface Props {
    assistantId?: string;
    openai?: OpenAI;
}
export function useOpenAiAssistant({ assistantId = '', openai: openai_arg }: Props) {
  const [messages, setMessages] = useState<Message[]>([ ]);
  const [input, setInput] = useState('');
  const [threadId, setThreadId] = useState<string | undefined>(undefined);
  const [status, setStatus] = useState<AssistantStatus>('awaiting_message');
  const [error, setError] = useState<undefined | Error>(undefined);
  const openai_ref = useRef(openai_arg ?? new OpenAI({
    dangerouslyAllowBrowser: true,
  }));

  const openai = openai_ref.current;
  //   const assistant = await openai.beta.assistants.create({
  //     name: 'Math Tutor',
  //     instructions: 'You are a personal math tutor. Write and run code to answer math questions.',
  //     model: 'gpt-4o'
  //   });



  const handleInputChange = (
    event:
      | React.ChangeEvent<HTMLInputElement>
      | React.ChangeEvent<HTMLTextAreaElement>,
  ) => {
    setInput(event.target.value);
  };

  const append = async (
    message: CreateMessage,
    requestOptions?: {
        data?: Record<string, string>;
      },
  ) => {
    try {
      setStatus('in_progress');
      console.log('requestOptions', requestOptions);

      let local_threadId = threadId;
      if (!local_threadId) {
        const thread = await openai.beta.threads.create();
        local_threadId = thread.id;
        setThreadId(local_threadId);
      }

      console.log('threadId', local_threadId);

      const created_message = await openai.beta.threads.messages.create(
        local_threadId,
        message
      );
      setMessages(messages => [
        ...messages,
        created_message,
      ]);

      console.log('created message', created_message);


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
        .on('messageDone', (message: Message) => console.log('messageDone', message))
        .on('textCreated', (content: Text) => console.log('textCreated', content))
        .on('textDelta', (delta: TextDelta, snapshot: Text) => console.log('textDelta', { delta, snapshot }))
        .on('textDone', (content: Text, snapshot: Message) => console.log('textDone', { content, snapshot }))
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
    requestOptions?: {
      data?: Record<string, string>;
    },
  ) => {
    event?.preventDefault?.();

    if (input === '') {
      return;
    }

    append({ role: 'user', content: input }, requestOptions);
  };

  return { input, setInput, messages, setMessages, threadId, error, status, submitMessage, handleInputChange, append };
}