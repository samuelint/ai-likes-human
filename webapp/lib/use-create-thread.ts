import { useCallback } from 'react';
import { useRouter } from 'next/navigation';
import { useOpenaiClient } from './openai-client';
import { useListThreads } from './use-list-threads';
import { Thread, ThreadCreateParams } from 'openai/resources/beta/threads/threads.mjs';
import { ImageAttachment } from './image-attachment.type';
import { createUserMessage } from './service/message-factory';


interface Props {
  redirect?: boolean
}

type CreateNewThread = (messageContent?: string, imageAttachments?: ImageAttachment[]) => Promise<Thread>;
export function useCreateThread({ redirect }: Props = {}): CreateNewThread {
  const openai = useOpenaiClient();
  const router = useRouter();
  const { revalidate } = useListThreads();

  return useCallback<CreateNewThread>(async (messageContent, imageAttachments) => {
    const messages: ThreadCreateParams.Message[] = [];

    if (messageContent) {
      messages.push(createUserMessage({ input: messageContent, imageAttachments }));
    }
    const newThread = await openai.beta.threads.create({
      messages,
    });

    if (redirect) {
      router.push(`/thread/${newThread.id}`);
    } else {
      revalidate();
    }

    return newThread;
  }, [openai.beta.threads, revalidate, redirect, router]);
}