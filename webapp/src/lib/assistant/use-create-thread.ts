import { useCallback } from 'react';

import { useOpenaiClient } from './openai-client';
import { useListThreads } from './use-list-threads';
import { Thread, ThreadCreateParams } from 'openai/resources/beta/threads/threads.mjs';
import { ImageAttachment } from './image-attachment.type';
import { createUserMessage } from './message-factory';
import { useLocation } from 'wouter';


interface Props {
  redirect?: boolean
}

interface CreateNewThreadArgs {
  assistantId?: string
  messageContent?: string
  imageAttachments?: ImageAttachment[]
}

type CreateNewThread = (args?: CreateNewThreadArgs) => Promise<Thread>;
export function useCreateThread({ redirect }: Props = {}): CreateNewThread {
  const openai = useOpenaiClient();
  const [_, setLocation] = useLocation();
  const { revalidate } = useListThreads();

  return useCallback<CreateNewThread>(async ({ assistantId, messageContent, imageAttachments } = {}) => {
    const messages: ThreadCreateParams.Message[] = [];

    if (messageContent) {
      messages.push(createUserMessage({ input: messageContent, imageAttachments }));
    }
    const newThread = await openai.beta.threads.create({
      messages,
      metadata: {
        assistantId,
      }
    });

    revalidate();

    if (redirect) {
      setLocation(`/thread/${newThread.id}`);
    }

    return newThread;
  }, [openai.beta.threads, revalidate, redirect, setLocation]);
}